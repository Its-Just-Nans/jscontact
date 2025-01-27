"""get figures from RFC 9553"""

from pathlib import Path
from xml.dom import minidom
import requests


def main(rfc, output_dir="raws"):
    """get figures from RFC"""
    text = requests.get(f"https://www.rfc-editor.org/rfc/{rfc}.xml", timeout=5).text
    xml = minidom.parseString(text)
    figures = xml.getElementsByTagName("figure")
    out_dir = Path(output_dir)
    out_dir.mkdir(exist_ok=True)
    number = len(figures)
    for figure in figures:
        name = figure.attributes["pn"].value.split("-")[0]
        figure_number = figure.attributes["pn"].value.split("-")[-1]
        # pad using the number of figures
        correct_name = f"{name}_{figure_number.zfill(len(str(number)))}"
        out_path = out_dir.joinpath(f"{correct_name}.txt")
        code = figure.getElementsByTagName("sourcecode")[0].firstChild.nodeValue
        code = code.strip().rstrip()
        with open(out_path, "w", encoding="utf-8") as file:
            file.write(code)


if __name__ == "__main__":
    main("rfc9553", "rfc9553/raws")
