def main():
    # 1. ask user for string
    var = input("Enter a string:").upper()

    indexes = [0 for _ in range(128)]
    # 2. iterate over each character
    for i in var:
        # - add a counter for each character
        indexes[ord(i)] += 1

    # 3. find character with highest counter
    index = indexes.index(max(indexes))
    most_common = chr(index)

    # 4. print character
    print("'" + most_common + "'", max(indexes))

main()
