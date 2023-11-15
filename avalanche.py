one = "8a702c93b104cfb027492ea48ba4bd3199fb9ddc507a73afb9fb8b906567532a"
two = "baed17b64490f3b0d6826b1b918368c8b931354d8f7e3be39bbcb626ad9f860d"

def calc(one, two):
    one_bits = int(one, 16)
    two_bits = int(two, 16)

    result = int(one, 16) ^ int(two, 16) # convert to integers and xor them together
    result = '{:x}'.format(result)     # convert back to hexadecimal

    bin_result = bin(int(result, 16)).count("1")
    print(bin_result)

    final = bin_result / len(one)
    print(final)

calc(one, two)