// generated file, do not edit
{% for a in artifacts %}
{% if a.flower.is_some() -%}
import {{ a.image_name }}_flower from "@image/artifacts/{{ a.image_name }}_flower"
{%- endif %}
{% if a.feather.is_some() -%}
import {{ a.image_name }}_feather from "@image/artifacts/{{ a.image_name }}_feather"
{%- endif %}
{% if a.sand.is_some() -%}
import {{ a.image_name }}_sand from "@image/artifacts/{{ a.image_name }}_sand"
{%- endif %}
{% if a.goblet.is_some() -%}
import {{ a.image_name }}_goblet from "@image/artifacts/{{ a.image_name }}_goblet"
{%- endif %}
{% if a.head.is_some() -%}
import {{ a.image_name }}_head from "@image/artifacts/{{ a.image_name }}_head"
{%- endif %}
{% endfor %}

export default {
    {% for a in artifacts %}
    "{{ a.name_mona }}": {
        eng: "{{ a.name_mona }}",
        name2: "{{ a.name }}",
        nameLocale: {{a.name_locale}},
        minStar: {{ a.min_star }},
        maxStar: {{ a.max_star }},
    {% if a.effect1.is_some() -%}
        effect1: {{a.effect1.unwrap()}},
    {%- endif %}
    {% if a.effect2.is_some() -%}
        effect2: {{a.effect2.unwrap()}},
    {%- endif %}
    {% if a.effect3.is_some() -%}
        effect3: {{a.effect3.unwrap()}},
    {%- endif %}
    {% if a.effect4.is_some() -%}
        effect4: {{a.effect4.unwrap()}},
    {%- endif %}
    {% if a.effect5.is_some() -%}
        effect5: {{a.effect5.unwrap()}},
    {%- endif %}

        {% if a.flower.is_some() -%}
        flower: {
            text: {{a.flower.unwrap()}},
            url: {{ a.image_name }}_flower,
        },
        {%- endif %}
        {% if a.feather.is_some() -%}
        feather: {
            text: {{a.feather.unwrap()}},
            url: {{ a.image_name }}_feather,
        },
        {%- endif %}
        {% if a.sand.is_some() -%}
        sand: {
            text: {{a.sand.unwrap()}},
            url: {{ a.image_name }}_sand,
        },
        {%- endif %}
        {% if a.goblet.is_some() -%}
        cup: {
            text: {{a.goblet.unwrap()}},
            url: {{ a.image_name }}_goblet,
        },
        {%- endif %}
        {% if a.head.is_some() -%}
        head: {
            text: {{a.head.unwrap()}},
            url: {{ a.image_name }}_head,
        },
        {%- endif %}
        config4: [
            {% for config in a.config4 %}
            {{ config|e("none") }},
            {% endfor %}
        ],
        config2: [
            {% for config in a.config2 %}
            {{ config|e("none") }},
            {% endfor %}
        ],
    },
    {% endfor %}
}
