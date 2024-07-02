### This program allows you to extract json from json. The output is the path to the json file and the keys by which the values ​​will be retrieved.

#### Use like this 
`./extract_json path/to/file.json first_key second_key`

# Example

data.json looks like this
	```
        {
            "name": "John Doe",
            "age": 30,
            "email": "john.doe@example.com",
            "phones": ["123-456-7890", "987-654-3210"],
            "product": {
                "title": "qwe"
            }
        }
	```

If run programm `./extract_json data.json name product`

We will receive such data ```
        {
            "name": "John Doe",
            "product": {
                "title": "qwe"
            }
        }
	```