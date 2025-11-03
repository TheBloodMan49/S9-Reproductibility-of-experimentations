from jinja2 import Template

template_content = open("property_template.py.jinja").read()
template = Template(template_content)

# Define parameters for the template
factors = {
    "method": "iterative",
    "big": True,
    "approximate_decimals": 5,
    "years": 50
}

# Render the template with the defined parameters
rendered_code = template.render(**factors)

# Write the rendered code to a new Python file
with open("generated_property_test.py", "w") as f:
    f.write(rendered_code)

print("Generated 'generated_property_test.py' with the specified parameters.")