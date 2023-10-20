import os
import json
from urllib.parse import urlparse

input_directory = "/Users/horst/Coding/rust/freeflowfeeds/data"
output_directory = "/Users/horst/Coding/rust/freeflowfeeds/newdata"

publishers = {
    "ef-magazin.de": "EFMAGAZIN",
    "freiheitsfunken.info": "FREIHEITSFUNKEN",
    "www.misesde.org": "MISESDE",
    "schweizermonat.ch": "SCHWEIZER_MONAT",
    "hayek-institut.at": "HAYEK_INSTITUT"
}

for filename in os.listdir(input_directory):
    if filename.endswith(".json"):
        file_path = os.path.join(input_directory, filename)

        with open(file_path, "r") as json_file:
            data = json.load(json_file)

            if "article" in data and "link" in data["article"]:
                link = data["article"]["link"]
                try:
                    host = urlparse(link).hostname
                except Exception as e:
                    print(f"Error parsing URL in file {file_path}: {str(e)}")
                    continue

                print(host)

                if host in publishers:
                    data["publisher"] = publishers[host]

                    output_path = os.path.join(output_directory, filename)

                    with open(output_path, "w") as output_file:
                        json.dump(data, output_file)

print("finished")
