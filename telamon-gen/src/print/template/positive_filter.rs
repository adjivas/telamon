{{#with Switch~}}
    {{#each cases~}}
        if {{../var}}.intersects({{this.[0]}}) { {{>positive_filter this.[1]}} }
    {{/each~}}
{{/with~}}
{{#with AllowValues~}}
    {{var}}.insert({{values}});
{{/with~}}
{{#with AllowAll~}}
    {{var}}.insert({{value_type}}::ALL);
{{/with~}}
{{#with Rules~}}
    let mut {{new_var}} = {{value_type}}::ALL;
    {{#each rules~}}
        {{>rule}}
    {{/each~}}
    {{old_var}}.insert({{new_var}});
{{/with~}}
