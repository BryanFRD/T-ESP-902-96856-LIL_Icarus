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
import os
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Dict, List, Tuple


def parse_lcov(lcov_path: str, project_root: str) -> Dict[str, List[Tuple[int, bool]]]:
    """
    Parse LCOV file and extract coverage data with relative paths.

    Args:
        lcov_path: Path to LCOV file
        project_root: Project root directory for converting to relative paths

    Returns:
        Dictionary mapping relative file paths to list of (line_number, is_covered) tuples
    """
    coverage_data: Dict[str, List[Tuple[int, bool]]] = {}
    current_file: str | None = None
    project_root_path = Path(project_root).resolve()

    with open(lcov_path, 'r', encoding='utf-8') as f:
        for line in f:
            line = line.strip()

            if line.startswith('SF:'):
                # Source file - convert to relative path
                absolute_path = line[3:]  # Remove 'SF:' prefix
                try:
                    # Convert absolute path to relative path from project root
                    relative_path = Path(absolute_path).resolve().relative_to(project_root_path)
                    current_file = str(relative_path)
                    coverage_data[current_file] = []
                except ValueError:
                    # Path is not relative to project root, keep as is
                    current_file = absolute_path
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


def convert_lcov_to_generic_coverage(lcov_path: str, output_path: str, project_root: str) -> None:
    """
    Convert LCOV file to SonarQube Generic Coverage XML format.

    Args:
        lcov_path: Path to input LCOV file
        output_path: Path to output XML file
        project_root: Project root directory for relative paths
    """
    print(f"Reading LCOV file: {lcov_path}")
    print(f"Project root: {project_root}")
    coverage_data = parse_lcov(lcov_path, project_root)

    total_files = len(coverage_data)
    total_lines = sum(len(lines) for lines in coverage_data.values())
    covered_lines = sum(
        sum(1 for _, is_covered in lines if is_covered)
        for lines in coverage_data.values()
    )

    print(f"Parsed coverage data:")
    print(f"  - {total_files} files")
    print(f"  - {total_lines} lines")
    if total_lines > 0:
        print(f"  - {covered_lines} covered lines ({covered_lines * 100 / total_lines:.1f}% coverage)")
    else:
        print(f"  - No lines to cover")

    print(f"Creating Generic Coverage XML: {output_path}")
    root = create_generic_coverage_xml(coverage_data)
    indent_xml(root)

    # Create XML tree and write to file with explicit declaration
    tree = ET.ElementTree(root)
    ET.register_namespace('', '')  # Avoid namespace prefixes
    
    with open(output_path, 'w', encoding='UTF-8') as f:
        f.write('<?xml version="1.0" encoding="UTF-8"?>\n')
        tree_str = ET.tostring(root, encoding='unicode')
        f.write(tree_str)

    print(f"Successfully created Generic Coverage XML: {output_path}")


def main() -> int:
    """Main entry point."""
    if len(sys.argv) < 3 or len(sys.argv) > 4:
        print("Usage: lcov_to_generic_coverage.py <input_lcov> <output_xml> [project_root]")
        print("Example: lcov_to_generic_coverage.py coverage/lcov.info coverage/generic-coverage.xml")
        print("         lcov_to_generic_coverage.py coverage/lcov.info coverage/generic-coverage.xml /builds/project")
        print("")
        print("If project_root is not specified, the current working directory is used.")
        return 1

    lcov_path = sys.argv[1]
    output_path = sys.argv[2]
    project_root = sys.argv[3] if len(sys.argv) == 4 else os.getcwd()

    # Validate input file exists
    if not Path(lcov_path).exists():
        print(f"Error: Input LCOV file not found: {lcov_path}")
        return 1

    # Create output directory if needed
    output_dir = Path(output_path).parent
    output_dir.mkdir(parents=True, exist_ok=True)

    try:
        convert_lcov_to_generic_coverage(lcov_path, output_path, project_root)
        return 0
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        return 1


if __name__ == '__main__':
    sys.exit(main())
