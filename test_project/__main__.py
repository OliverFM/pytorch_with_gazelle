if __name__ == "__main__":
    from main import main

    import torch

    tensor = torch.ones((1, 2, 3))
    print(f"testing torch {tensor=}")

    main()
