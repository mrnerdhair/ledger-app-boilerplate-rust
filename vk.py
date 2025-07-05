import json

def dec_to_hex_array(dec_str):
    """
    Converts a decimal string to a list of 64-bit hexadecimal strings.

    Args:
        dec_str: The decimal number as a string.

    Returns:
        A list of formatted 64-bit hexadecimal strings.
    """
    if not dec_str.isdigit():
        return []
    # Convert decimal string to an integer
    n = int(dec_str)
    # Convert integer to a hexadecimal string, removing the '0x' prefix
    hex_val = hex(n)[2:]
    # Pad with leading zeros to ensure it's 64 characters long (for 256 bits)
    padded_hex = hex_val.zfill(64)
    # Split the 64-character hex string into four 16-character (64-bit) chunks
    return [f"0x{padded_hex[i:i+16]}" for i in range(0, 64, 16)]

def format_g1(data):
    """
    Formats a G1 point into the Rust G1Affine structure with specific formatting
    for BigInt arrays.
    """
    if len(data) < 2:
        return ""
    
    x_hex = dec_to_hex_array(data[0])
    y_hex = dec_to_hex_array(data[1])

    # Define the indentation for the hex values inside the array
    indent = "                    " # 20 spaces

    # Reverse the list of hex strings and join them with newlines and indentation
    x_formatted = ",\n".join([f"{indent}{val}" for val in reversed(x_hex)])
    y_formatted = ",\n".join([f"{indent}{val}" for val in reversed(y_hex)])

    # Return the fully formatted G1Affine string
    return f"""G1Affine::new(
                BaseField::new(BigInt::new([
{x_formatted}
                ])),
                BaseField::new(BigInt::new([
{y_formatted}
                ])),
            )"""

def format_g2(data):
    """
    Formats a G2 point into the Rust G2Affine structure with specific formatting
    for BigInt arrays.
    """
    if len(data) < 2 or len(data[0]) < 2 or len(data[1]) < 2:
        return ""

    # Extract and convert all coordinates for the G2 point
    x_c0_hex = dec_to_hex_array(data[0][0])
    x_c1_hex = dec_to_hex_array(data[0][1])
    y_c0_hex = dec_to_hex_array(data[1][0])
    y_c1_hex = dec_to_hex_array(data[1][1])

    # Define the deeper indentation for the hex values in G2
    indent = "                        " # 24 spaces

    # Reverse and format each list of hex strings
    x_c0_formatted = ",\n".join([f"{indent}{val}" for val in reversed(x_c0_hex)])
    x_c1_formatted = ",\n".join([f"{indent}{val}" for val in reversed(x_c1_hex)])
    y_c0_formatted = ",\n".join([f"{indent}{val}" for val in reversed(y_c0_hex)])
    y_c1_formatted = ",\n".join([f"{indent}{val}" for val in reversed(y_c1_hex)])

    # Return the fully formatted G2Affine string
    return f"""G2Affine::new(
                Fp2::new(
                    BaseField::new(BigInt::new([
{x_c0_formatted}
                    ])),
                    BaseField::new(BigInt::new([
{x_c1_formatted}
                    ])),
                ),
                Fp2::new(
                    BaseField::new(BigInt::new([
{y_c0_formatted}
                    ])),
                    BaseField::new(BigInt::new([
{y_c1_formatted}
                    ])),
                ),
            )"""

# --- Main execution block ---

# Input JSON data is now stored in a variable
input_data = {
  "protocol": "groth16",
  "curve": "bn128",
  "nPublic": 1,
  "vk_alpha_1": [
    "20491192805390485299153009773594534940189261866228447918068658471970481763042",
    "9383485363053290200918347156157836566562967994039712273449902621266178545958",
    "1"
  ],
  "vk_beta_2": [
    [
      "6375614351688725206403948262868962793625744043794305715222011528459656738731",
      "4252822878758300859123897981450591353533073413197771768651442665752259397132"
    ],
    [
      "10505242626370262277552901082094356697409835680220590971873171140371331206856",
      "21847035105528745403288232691147584728191162732299865338377159692350059136679"
    ],
    [
      "1",
      "0"
    ]
  ],
  "vk_gamma_2": [
    [
      "10857046999023057135944570762232829481370756359578518086990519993285655852781",
      "11559732032986387107991004021392285783925812861821192530917403151452391805634"
    ],
    [
      "8495653923123431417604973247489272438418190587263600148770280649306958101930",
      "4082367875863433681332203403145435568316851327593401208105741076214120093531"
    ],
    [
      "1",
      "0"
    ]
  ],
  "vk_delta_2": [
    [
      "719732304806420095012598419160281616958726522641871716346703441882108680247",
      "20732181798283163993429430179862607382802007833352280239743278885065761175433"
    ],
    [
      "14532359612094420789795070171272619166898039937715276988337464250879148233525",
      "553288750660617101326269548529101295779988757712216100129422862708929566763"
    ],
    [
      "1",
      "0"
    ]
  ],
  "vk_alphabeta_12": [
    [
      [
        "2029413683389138792403550203267699914886160938906632433982220835551125967885",
        "21072700047562757817161031222997517981543347628379360635925549008442030252106"
      ],
      [
        "5940354580057074848093997050200682056184807770593307860589430076672439820312",
        "12156638873931618554171829126792193045421052652279363021382169897324752428276"
      ],
      [
        "7898200236362823042373859371574133993780991612861777490112507062703164551277",
        "7074218545237549455313236346927434013100842096812539264420499035217050630853"
      ]
    ],
    [
      [
        "7077479683546002997211712695946002074877511277312570035766170199895071832130",
        "10093483419865920389913245021038182291233451549023025229112148274109565435465"
      ],
      [
        "4595479056700221319381530156280926371456704509942304414423590385166031118820",
        "19831328484489333784475432780421641293929726139240675179672856274388269393268"
      ],
      [
        "11934129596455521040620786944827826205713621633706285934057045369193958244500",
        "8037395052364110730298837004334506829870972346962140206007064471173334027475"
      ]
    ]
  ],
  "IC": [
    [
      "6819801395408938350212900248749732364821477541620635511814266536599629892365",
      "9092252330033992554755034971584864587974280972948086568597554018278609861372",
      "1"
    ],
    [
      "17882351432929302592725330552407222299541667716607588771282887857165175611387",
      "18907419617206324833977586007131055763810739835484972981819026406579664278293",
      "1"
    ]
  ]
}

# Start building the Rust code string
rust_code = "VerifyingKey::<Bn254> {\n"

# Process vk_alpha_1
if 'vk_alpha_1' in input_data:
    rust_code += f"    alpha_g1: {format_g1(input_data['vk_alpha_1'])},\n"

# Process vk_beta_2
if 'vk_beta_2' in input_data:
    rust_code += f"    beta_g2: {format_g2(input_data['vk_beta_2'])},\n"

# Process vk_gamma_2
if 'vk_gamma_2' in input_data:
    rust_code += f"    gamma_g2: {format_g2(input_data['vk_gamma_2'])},\n"

# Process vk_delta_2
if 'vk_delta_2' in input_data:
    rust_code += f"    delta_g2: {format_g2(input_data['vk_delta_2'])},\n"

# Process IC (renamed to gamma_abc_g1)
if 'IC' in input_data:
    ic_formatted = ",\n".join([f"        {format_g1(item)}" for item in input_data['IC']])
    rust_code += f"    gamma_abc_g1: vec![\n{ic_formatted}\n    ],\n"

rust_code += "}"

# Print the final Rust code to the console
print(rust_code)
