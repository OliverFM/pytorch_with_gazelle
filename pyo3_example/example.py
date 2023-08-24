from example_lib import sum_as_string  # gazelle: ignore


def main():
    result = sum_as_string(1, 4)
    assert result == "5"
    print(f"{result=}")


if __name__ == "__main__":
    main()
