import argparse
from bs4 import BeautifulSoup
from collections import namedtuple

Table = namedtuple('Table', 'headers, rows')


def main():
    args = _parse_args()
    soup = BeautifulSoup(open(args.input).read(), 'html.parser')
    lines = [_to_md(_parse_table(x)) for x in soup.find_all('table')]

    if args.output:
        open(args.output, 'w').write('\n\n'.join(lines))
    else:
        print('\n\n'.join(lines))


def _parse_args():
    parser = argparse.ArgumentParser(
        "A handful cli to convert html table to markdown table")
    parser.add_argument('input', help='input html file')
    parser.add_argument(
        '--output', help='output markdown file (default to stdout)')
    return parser.parse_args()


def _parse_table(tb) -> Table:
    headers = [x.text for x in tb.select('thead th')]
    rows = []
    for r in tb.select('tbody tr'):
        row = [x.text for x in r.select('td')]
        rows.append(row)

    return Table(headers, rows)


def _to_md(tb: Table) -> str:
    header_line = '|' + ' | '.join(x for x in tb.headers) + '|'
    sep = '|' + ' | '.join('---' for _ in tb.headers) + '|'
    rows = '\n'.join('|' + ' | '.join(r) + '|' for r in tb.rows)
    return '\n'.join([header_line, sep, rows])


if __name__ == '__main__':
    main()
