# import ipdb; ipdb.set_trace()
import sys
import os
# print(sys.version)
# print(sys.path)
# print(os.listdir('pyo3_example'))
# sys.path.append(os.getcwd() + "/pyo3_example")
# sys.path.append(os.getcwd() + "/pyo3_example/example_lib.so")
# print(sys.path)
from  example_lib import sum_as_string

def main():
    result = sum_as_string(1,4)
    assert(result == "5")
    print(f"{result=}")

if __name__ == "__main__":
    main()
