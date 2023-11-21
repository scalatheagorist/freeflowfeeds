import json
import os
import time

input_directory = "/Users/horst/Coding/rust/freeflowfeeds/data"
output_directory = "/Users/horst/Coding/rust/freeflowfeeds/data"

language = "DE"

for filename in os.listdir(input_directory)[::-1]:
    if filename.endswith(".json"):
        file_path = os.path.join(input_directory, filename)

        with open(file_path, "r") as json_file:
            data = json.load(json_file)

            if "article" in data and "link" in data["article"]:
                data["lang"] = language

                output_path = os.path.join(output_directory, filename)

                # manipulate modified date
                time.sleep(0.1)

                with open(output_path, "w") as output_file:
                    json.dump(data, output_file)

print("finished")
