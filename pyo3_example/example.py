from ext import sum_as_string


def main():
    result = sum_as_string(1,4)
    assert(result == "5")
    print(f"{result=}")

if __name__ == "__main__":
    main()
