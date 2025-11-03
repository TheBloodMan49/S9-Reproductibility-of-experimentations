from jinja2 import Template

template_content = open("property_template.py.jinja").read()
template = Template(template_content)

# Define parameters for the template
factors = [
    { "method": "iterative", "big": True, "approximate_decimals": 5, "years": 50},
    { "method": "iterative", "big": False, "approximate_decimals": 5, "years": 50},
    { "method": "iterative", "big": True, "approximate_decimals": 100, "years": 50},
    { "method": "iterative", "big": False, "approximate_decimals": 100, "years": 50},
    { "method": "iterative", "big": True, "approximate_decimals": 300, "years": 50},
    { "method": "iterative", "big": False, "approximate_decimals": 300, "years": 50},
    
    { "method": "logarithmic", "big": True, "approximate_decimals": 5, "years": 50},
    { "method": "logarithmic", "big": False, "approximate_decimals": 5, "years": 50},
    { "method": "logarithmic", "big": True, "approximate_decimals": 100, "years": 50},
    { "method": "logarithmic", "big": False, "approximate_decimals": 100, "years": 50},
    { "method": "logarithmic", "big": True, "approximate_decimals": 300, "years": 50},
    { "method": "logarithmic", "big": False, "approximate_decimals": 300, "years": 50},
]

for factors_set in factors:
    # Render the template with the defined parameters
    rendered_code = template.render(**factors_set)

    # Write the rendered code to a new Python file
    with open("generated_property_test.py", "w") as f:
        f.write(rendered_code)

    print("Running 'generated_property_test.py' with the specified parameters.")
    # Execute the generated code
    exec(rendered_code)