from cexample import operate
import sys

if __name__ == '__main__':
    print(operate('-', 7, 3))
    assert operate('-', 7, 3) == 4
    try:
        operate('?', 7, 3)
        print("FAILURE")
        sys.exit(-1)
    except:
        print("SUCCESS")