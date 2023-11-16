import random
one = "8a702c93b104cfb027492ea48ba4bd3199fb9ddc507a73afb9fb8b906567532a"
two = "baed17b64490f3b0d6826b1b918368c8b931354d8f7e3be39bbcb626ad9f860d"

def calc(one, two):
    one_bits = int(one, 16)
    two_bits = int(two, 16)

    result = int(one, 16) ^ int(two, 16) # convert to integers and xor them together
    result = '{:x}'.format(result)     # convert back to hexadecimal

    bin_result = bin(int(result, 16)).count("1")
    final = bin_result / len(one)
    final *= 10
    print(final)
    return final

def main():
    list_vals = []
    file1 = open('test.txt', 'r')
    lines = file1.readlines()

    for line1 in lines:
        line2 = flip_rand(line1)
        list_vals.append(calc(line1, line2))


def flip_rand(hex_string):
    # Convert hex string to byte array
    byte_array = bytearray.fromhex(hex_string)

    # Choose a random index
    random_index = random.randint(0, len(byte_array) - 1)

    # Flip the chosen byte
    byte_array[random_index] ^= 0xFF

    # Convert the modified byte array back to hex string
    modified_hex_string = byte_array.hex()

    return modified_hex_string

main()
calc(one, two)