#!/usr/bin/env python3
"""
Convert LCOV format to SonarQube Generic Test Coverage XML format.

LCOV format:
    SF:path/to/file.rs
    DA:line_number,hit_count
    end_of_record

Generic Coverage XML format:
    <?xml version="1.0" encoding="UTF-8"?>
    <coverage version="1">
      <file path="path/to/file.rs">
        <lineToCover lineNumber="5" covered="true"/>
        <lineToCover lineNumber="10" covered="false"/>
      </file>
    </coverage>
"""

import sys
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Dict, List, Tuple


def parse_lcov(lcov_path: str) -> Dict[str, List[Tuple[int, bool]]]:
    """
    Parse LCOV file and extract coverage data.

    Returns:
        Dictionary mapping file paths to list of (line_number, is_covered) tuples
    """
    coverage_data: Dict[str, List[Tuple[int, bool]]] = {}
    current_file: str | None = None

    with open(lcov_path, 'r', encoding='utf-8') as f:
        for line in f:
            line = line.strip()

            if line.startswith('SF:'):
                # Source file
                current_file = line[3:]  # Remove 'SF:' prefix
                coverage_data[current_file] = []

            elif line.startswith('DA:'):
                # Data line: DA:line_number,hit_count
                if current_file is None:
                    continue

                parts = line[3:].split(',')  # Remove 'DA:' prefix and split
                if len(parts) == 2:
                    line_number = int(parts[0])
                    hit_count = int(parts[1])
                    is_covered = hit_count > 0
                    coverage_data[current_file].append((line_number, is_covered))

    return coverage_data


def create_generic_coverage_xml(coverage_data: Dict[str, List[Tuple[int, bool]]]) -> ET.Element:
    """
    Create Generic Coverage XML structure from coverage data.

    Returns:
        XML Element root
    """
    root = ET.Element('coverage', version='1')

    for file_path, lines in sorted(coverage_data.items()):
        if not lines:
            continue

        file_elem = ET.SubElement(root, 'file', path=file_path)

        # Sort lines by line number
        for line_number, is_covered in sorted(lines):
            ET.SubElement(
                file_elem,
                'lineToCover',
                lineNumber=str(line_number),
                covered='true' if is_covered else 'false'
            )

    return root


def indent_xml(elem: ET.Element, level: int = 0) -> None:
    """Add indentation to XML for pretty printing."""
    indent = "  "
    i = "\n" + level * indent
    if len(elem):
        if not elem.text or not elem.text.strip():
            elem.text = i + indent
        if not elem.tail or not elem.tail.strip():
            elem.tail = i
        for child in elem:
            indent_xml(child, level + 1)
        if not child.tail or not child.tail.strip():
            child.tail = i
    else:
        if level and (not elem.tail or not elem.tail.strip()):
            elem.tail = i


def convert_lcov_to_generic_coverage(lcov_path: str, output_path: str) -> None:
    """
    Convert LCOV file to SonarQube Generic Coverage XML format.

    Args:
        lcov_path: Path to input LCOV file
        output_path: Path to output XML file
    """
    print(f"Reading LCOV file: {lcov_path}")
    coverage_data = parse_lcov(lcov_path)

    total_files = len(coverage_data)
    total_lines = sum(len(lines) for lines in coverage_data.values())
    covered_lines = sum(
        sum(1 for _, is_covered in lines if is_covered)
        for lines in coverage_data.values()
    )

    print(f"Parsed coverage data:")
    print(f"  - {total_files} files")
    print(f"  - {total_lines} lines")
    print(f"  - {covered_lines} covered lines ({covered_lines * 100 / total_lines:.1f}% coverage)")

    print(f"Creating Generic Coverage XML: {output_path}")
    root = create_generic_coverage_xml(coverage_data)
    indent_xml(root)

    # Create XML tree and write to file
    tree = ET.ElementTree(root)
    ET.register_namespace('', '')  # Avoid namespace prefixes
    tree.write(
        output_path,
        encoding='UTF-8',
        xml_declaration=True,
        method='xml'
    )

    print(f"Successfully created Generic Coverage XML: {output_path}")


def main() -> int:
    """Main entry point."""
    if len(sys.argv) != 3:
        print("Usage: lcov_to_generic_coverage.py <input_lcov> <output_xml>")
        print("Example: lcov_to_generic_coverage.py coverage/lcov.info coverage/generic-coverage.xml")
        return 1

    lcov_path = sys.argv[1]
    output_path = sys.argv[2]

    # Validate input file exists
    if not Path(lcov_path).exists():
        print(f"Error: Input LCOV file not found: {lcov_path}")
        return 1

    # Create output directory if needed
    output_dir = Path(output_path).parent
    output_dir.mkdir(parents=True, exist_ok=True)

    try:
        convert_lcov_to_generic_coverage(lcov_path, output_path)
        return 0
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        return 1


if __name__ == '__main__':
    sys.exit(main())
