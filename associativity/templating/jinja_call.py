from jinja2 import Template

template_content = open("property_template.py.jinja").read()
template = Template(template_content)

# Define parameters for the template
factors = {
    "min_value": -100.0,
    "max_value": 100.0,
    "operation1": "x + (y + z)",
    "operation2": "(x + y) + z",
    "tolerance": 0,
    "repetitions": 10000
}

# Render the template with the defined parameters
rendered_code = template.render(**factors)

# Write the rendered code to a new Python file
with open("generated_property_test.py", "w") as f:
    f.write(rendered_code)

print("Generated 'generated_property_test.py' with the specified parameters.")