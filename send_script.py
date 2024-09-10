import json
import requests

# Load the JSON file
with open('model.json', 'r') as file:
    data = json.load(file)

# Server URL
url = 'http://localhost:8080/welcome'  # Adjust the URL and port as needed

# Send POST request
try:
    response = requests.post(url, json=data)
    
    # Check the response
    if response.status_code == 201:
        print("Welcome created successfully")
    else:
        print(f"Error: {response.status_code} - {response.text}")
except requests.exceptions.RequestException as e:
    print(f"An error occurred: {e}")