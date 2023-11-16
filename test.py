from math import log2

def hamming_distance(hash1, hash2):
    # Calculate the Hamming distance between two hash values
    # print(bin(int(hash1, 16) ^ int(hash2, 16)).count('1'))
    return bin(int(hash1, 16) ^ int(hash2, 16)).count('1')


def avalanche_coefficient(hash_values):
    total_pairs = 0
    differing_bits = 0

    # Compare all pairs of hash values
    for i in range(len(hash_values)):
        for j in range(i + 1, len(hash_values)):
            total_pairs += 1
            differing_bits += hamming_distance(hash_values[i], hash_values[j])
            print(total_pairs)

    # Calculate the avalanche coefficient
    coefficient = differing_bits / (total_pairs * 256.0)
    return coefficient

def calculate_entropy(hash_values):
    total_hashes = len(hash_values)
    hash_length = len(hash_values[0])  # Assuming all hash values have the same length

    entropy = 0.0

    for i in range(hash_length):
        char_count = {char: 0 for char in set([hash_value[i] for hash_value in hash_values])}

        for hash_value in hash_values:
            char_count[hash_value[i]] += 1

        for char, count in char_count.items():
            probability = count / total_hashes
            entropy -= probability * log2(probability)

    return entropy




def main():
    file_path = "test.txt"  # Replace with the path to your text file
    with open(file_path, 'r') as file:
        hash_values = [line.strip() for line in file]

    if len(hash_values) < 2:
        print("Error: At least two hash values are required for avalanche coefficient calculation.")
        return

    coefficient = avalanche_coefficient(hash_values) * 100
    print(f"Avalanche Coefficient: {coefficient:.4f}%")

    entropy = calculate_entropy(hash_values)
    print(f"Entropy: {entropy:.4f}%")

if __name__ == "__main__":
    main()
