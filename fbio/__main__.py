
from . import nuc2bit

import random
import argparse

def main(argv):
    random.seed(42)
    
    parser = argparse.ArgumentParser('fbio')
    subparsers = parser.add_subparsers()
    
    nuc2bit_parser = subparsers.add_parser('nuc2bit')
    nuc2bit_parser.set_defaults(func=nuc2bit.main)
    nuc2bit_parser.add_argument('-o', '--output', dest='output')
    nuc2bit_parser.add_argument('-s', '--subsample_size', type=int, dest='subsample', default=50)
    nuc2bit_parser.add_argument('-f', '--filters', nargs='+', dest='filters')

    args = parser.parse_args(argv)
    args.func(args)
    

if __name__ == '__main__':
    import sys
    
    main(sys.argv[1:])
