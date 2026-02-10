# CDN 链接问题分析与修复方案

## 问题描述

UI 渲染时部分图片显示失败，根本原因是使用了无效的 CDN 链接。

## 发现

### 1. 数据源架构

**genshin_artifact 项目使用代码生成方式管理静态数据：**

| 组件 | 路径 | 作用 |
|------|------|------|
| 模板文件 | `mona_generate/templates/*.js` | 定义生成模板 |
| Rust 生成器 | `mona_generate/src/gen_meta/*.rs` | 从 AnimeGameData 读取数据并填充模板 |
| 生成文件 | `src/assets/_gen_*.js` | 运行时使用的静态数据 |
| 本地图片 | `src/images/{characters,artifacts,weapons}/` | 本地化图片资源 |

**数据流：**
```
AnimeGameData (官方JSON) → Rust生成器 → _gen_*.js (模板渲染) → Vue组件 (渲染)
```

### 2. CDN 链接来源

**模板文件中的硬编码 CDN：**

**character_meta_template.js:**
```javascript
const template = "https://upload-bbs.mihoyo.com/game_record/genshin/character_icon/UI_AvatarIcon_#.png"
const newTemplate = "https://act-webstatic.mihoyo.com/hk4e/e20200928calculate/item_icon_u9b0pg/#.png"
```

**artifact_meta_template.js:**
```javascript
const template = "https://upload-bbs.mihoyo.com/game_record/genshin/equip/#.png"
const newTemplate = "https://act-webstatic.mihoyo.com/hk4e/e20200928calculate/item_icon_u9b0pg/#.png"
```

### 3. CDN 可用性检测结果

| CDN | 状态 | 说明 |
|-----|------|------|
| `upload-bbs.mihoyo.com/game_record/genshin/equip/` | ✅ 200 | 旧圣遗物图标正常 |
| `upload-bbs.mihoyo.com/game_record/genshin/character_icon/` | ❌ 302→404 | 角色图标失效 |
| `act-webstatic.mihoyo.com/hk4e/.../item_icon_u9b0pg/` | ❌ 404 | 新圣遗物/角色失效 |

### 4. 角色头像现状

**角色图片已迁移到本地：**
- 模板已修改为使用 `@image/characters/{name}_avatar`
- `_gen_character.js` 中 `avatar` 字段指向本地导入
- 244 个本地角色图片存在

```javascript
// character_meta_template.js 生成结果
import Aino_avatar from "@image/characters/Aino_avatar"
export default {
    Aino: {
        avatar: Aino_avatar,  // 本地图片引用
    }
}
```

### 5. 圣遗物图标问题

**当前问题：**
- `_gen_artifact.js` 仍使用 CDN URL
- ~~约 20+ 个新圣遗物缺少本地图片~~ ✅ **已解决**

**2026-02-08 更新：** 使用 genshin-data-research skill 的 `download_images.py` 脚本成功下载了所有缺失的圣遗物图片：

```
✓ AubadeOfMorningstarAndMoon (5/5)
✓ DeepwoodMemories (5/5)
✓ FinaleOfTheDeepGalleries (5/5)
✓ FlowerOfParadiseLost (5/5)
✓ FragmentOfHarmonicWhimsy (5/5)
✓ GildedDreams (5/5)
✓ GoldenTroupe (5/5)
✓ LongNightsOath (5/5)
✓ MarechausseeHunter (5/5)
✓ NightOfTheSkysUnveiling (5/5)
✓ NighttimeWhispersInTheEchoingWoods (5/5)
✓ NymphsDream (5/5)
✓ ObsidianCodex (5/5)
✓ ScrollOfTheHeroOfCinderCity (5/5)
✓ SilkenMoonsSerenade (5/5)
✓ SongOfDaysPast (5/5)
✓ UnfinishedReverie (5/5)
✓ VourukashasGlow (5/5)

总计: 18 个圣遗物 × 5 = 90 张新图片
```

**下载命令：**
```bash
cd /home/moltbot/moltbot-app/skills/genshin-data-research/scripts
python3 download_images.py "Artifact Name" --type artifact --output /mnt/e/Moltbot/workspace/genshin_artifact/src/images/artifacts/
```

**结果：**
- 原有图片: 189 张
- 下载后: 284 张
- 新增: 95 张图片

### 6. 图片命名规范

**本地图片结构：**
```
src/images/
├── characters/
│   ├── {Name}_avatar.png      # 角色头像
│   └── {Name}_splash.webp     # 角色立绘
├── artifacts/
│   ├── {ArtifactName}_flower.png
│   ├── {ArtifactName}_feather.png
│   ├── {ArtifactName}_sand.png
│   ├── {ArtifactName}_goblet.png
│   └── {ArtifactName}_head.png
└── weapons/
    └── {WeaponName}_tn.png    # 武器图标
```

## 已完成的修复

### 1. ~~下载缺失的圣遗物图片~~ ✅ 2026-02-08

使用 genshin-data-research skill 下载：
```bash
python3 download_images.py "Artifact Name" --type artifact
```

**结果：**
- 原有图片: 189 张
- 下载后: 284 张
- 新增: 95 张图片 (18 个圣遗物 × 5 件)

### 2. 创建 URL 解析器

**文件：** `src/utils/artifact_url_resolver.js`

```javascript
// 已知失效的 hash → 本地路径映射
const HASH_TO_LOCAL = {
  '9c3e75b95befcea2afa110828a2b5679': '/images/artifacts/GoldenTroupe_flower.png',
  // ... 更多映射
};

export function resolveArtifactUrl(hash, artifactName, position) {
  if (HASH_TO_LOCAL[hash]) {
    return HASH_TO_LOCAL[hash];
  }
  return `https://upload-bbs.mihoyo.com/game_record/genshin/equip/${hash}.png`;
}

export function getArtifactImageUrl(artifactName, position) {
  return `/images/artifacts/${artifactName}_${position}.png`;
}
```

### 2. 修复 ArtifactDisplay.vue

```typescript
import { getArtifactImageUrl } from "@/utils/artifact_url_resolver"

const imageSrc = computed(() => {
  const localUrl = getArtifactImageUrl(props.item.setName, props.item.position)
  return localUrl
})
```

### 3. 修复 utils.js

```javascript
import { getArtifactImageUrl } from "@/utils/artifact_url_resolver"

export function getArtifactThumbnailURL(setName) {
  return getArtifactImageUrl(setName, 'flower')
}
```

## 待完成

### 1. ~~下载缺失的圣遗物图片~~ ✅ 已完成

使用 genshin-data-research skill 的 `download_images.py` 脚本下载了 18 个新圣遗物共 95 张图片。

### 2. 更新模板文件

**artifact_meta_template.js** 需要修改为：
1. 导入本地图片
2. 移除 CDN URL 依赖
3. 使用本地图片路径

```javascript
{% for a in artifacts %}
import {{ a.name_mona }}_flower from "@image/artifacts/{{ a.name_mona }}_flower.png"
import {{ a.name_mona }}_feather from "@image/artifacts/{{ a.name_mona }}_feather.png"
import {{ a.name_mona }}_sand from "@image/artifacts/{{ a.name_mona }}_sand.png"
import {{ a.name_mona }}_goblet from "@image/artifacts/{{ a.name_mona }}_goblet.png"
import {{ a.name_mona }}_head from "@image/artifacts/{{ a.name_mona }}_head.png"
{% endfor %}

export default {
    {% for a in artifacts %}
    "{{ a.name_mona }}": {
        flower: {
            text: {{a.flower.unwrap()}},
            url: {{ a.name_mona }}_flower,  // 直接使用本地导入
        },
        // ...
    },
    {% endfor %}
}
```

```javascript
{% for a in artifacts %}
import {{ a.name_mona }}_flower from "@image/artifacts/{{ a.name_mona }}_flower.png"
import {{ a.name_mona }}_feather from "@image/artifacts/{{ a.name_mona }}_feather.png"
import {{ a.name_mona }}_sand from "@image/artifacts/{{ a.name_mona }}_sand.png"
import {{ a.name_mona }}_goblet from "@image/artifacts/{{ a.name_mona }}_goblet.png"
import {{ a.name_mona }}_head from "@image/artifacts/{{ a.name_mona }}_head.png"
{% endfor %}

export default {
    {% for a in artifacts %}
    "{{ a.name_mona }}": {
        flower: {
            text: {{a.flower.unwrap()}},
            url: {{ a.name_mona }}_flower,  // 直接使用本地导入
        },
        // ...
    },
    {% endfor %}
}
```

### 3. 重新生成元数据

```bash
cd /mnt/e/Moltbot/workspace/genshin_artifact
npm run gen_meta
```

### 4. 验证修复

```bash
# 启动开发服务器
npm run serve

# 检查控制台是否有图片加载错误
```

## 长期建议

### 1. 图片管理策略

**当前问题：** 模板中硬编码 CDN URL
**建议方案：**
1. 所有图片资源本地化
2. 使用 import 方式引用
3. 运行时按需加载

### 2. CDN URL 检测脚本

**已创建：** `scripts/cdn_link_checker.py`

```bash
python scripts/cdn_link_checker.py \
  --repo /mnt/e/Moltbot/workspace/genshin_artifact \
  --concurrency 20 \
  --timeout 15
```

功能：
- 扫描指定仓库
- 检测所有 CDN 链接可用性
- 有限并发避免请求过载
- JSON 结果输出

### 3. 图片下载脚本

**已创建：** `scripts/download_missing_artifact_images.py`

功能：
- 自动检测缺失图片
- 多 CDN 源 fallback
- 并发下载
- 进度报告

## 相关文件

| 文件 | 路径 | 说明 |
|------|------|------|
| CDN 检测脚本 | `/mnt/e/Moltbot/scripts/cdn_link_checker.py` | 检测 CDN 链接可用性 |
| 图片下载脚本 | `/mnt/e/Moltbot/scripts/download_missing_artifact_images.py` | 下载缺失图片 |
| URL 解析器 | `/mnt/e/Moltbot/workspace/genshin_artifact/src/utils/artifact_url_resolver.js` | CDN fallback |
| 角色模板 | `/mnt/e/Moltbot/workspace/genshin_artifact/mona_generate/templates/character_meta_template.js` | 已修复 |
| 圣遗物模板 | `/mnt/e/Moltbot/workspace/genshin_artifact/mona_generate/templates/artifact_meta_template.js` | 待修复 |

## 时间线

| 时间 | 操作 |
|------|------|
| 2026-02-08 16:43 | 检测到 CDN 链接失效 |
| 2026-02-08 17:09 | 创建 CDN 检测脚本 |
| 2026-02-08 17:11-17:21 | 分析代码架构，定位问题 |
| 2026-02-08 17:21-17:30 | 创建修复组件和脚本 |

## 结论

**问题根因：** 模板中硬编码的 CDN URL 失效

**已解决：**
- ✅ 角色头像已迁移到本地
- ✅ 新圣遗物图片已全部下载 (18个/95张)

**待解决：** 圣遗物图标仍依赖失效 CDN，需要：
1. ~~下载缺失图片~~ ✅
2. 更新模板使用本地图片
3. 重新生成元数据

### 图片下载统计

| 项目 | 下载前 | 下载后 | 新增 |
|------|--------|--------|------|
| 圣遗物图片 | 189 张 | 284 张 | 95 张 |
| 圣遗物数量 | ~45 个 | ~60 个 | 18 个 |
