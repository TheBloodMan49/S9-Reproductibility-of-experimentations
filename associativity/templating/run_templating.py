from jinja2 import Template

template_content = open("property_template.py.jinja").read()
template = Template(template_content)

# Define parameters for the template
factors = [
    { "min_value": -100.0, "max_value": 100.0, "operation1": "x + (y + z)", "operation2": "(x + y) + z", "tolerance": 0, "repetitions": 10000},
    { "min_value": -1e6, "max_value": 1e6, "operation1": "x * (y * z)", "operation2": "(x * y) * z", "tolerance": 1e-5, "repetitions": 5000},
    { "min_value": -1e3, "max_value": 1e3, "operation1": "x - (y - z)", "operation2": "(x - y) - z", "tolerance": 1e-3, "repetitions": 8000},
    { "min_value": -500.0, "max_value": 500.0, "operation1": "x / (y + 1e-6)", "operation2": "(x / y) + (x / 1e-6)", "tolerance": 1e-2, "repetitions": 6000},

    { "min_value": -100.0, "max_value": 100.0, "operation1": "x + y", "operation2": "y + x", "tolerance": 0, "repetitions": 10000},
    { "min_value": -1e6, "max_value": 1e6, "operation1": "x * y", "operation2": "y * x", "tolerance": 1e-5, "repetitions": 5000},
    { "min_value": -1e3, "max_value": 1e3, "operation1": "x - y", "operation2": "-(y - x)", "tolerance": 1e-3, "repetitions": 8000},
    { "min_value": -500.0, "max_value": 500.0, "operation1": "x / (y + 1e-6)", "operation2": "(y + 1e-6) / x", "tolerance": 1e-2, "repetitions": 6000},
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