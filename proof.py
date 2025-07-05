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

def format_g1(data, base_indent="            "):
    """
    Formats a G1 point into the Rust G1Affine structure with specific formatting
    for BigInt arrays.
    """
    if len(data) < 2:
        return ""
    
    x_hex = dec_to_hex_array(data[0])
    y_hex = dec_to_hex_array(data[1])

    # Define the indentation for the hex values inside the array
    indent = base_indent + "    " 

    # Reverse the list of hex strings and join them with newlines and indentation
    x_formatted = ",\n".join([f"{indent}{val}" for val in reversed(x_hex)])
    y_formatted = ",\n".join([f"{indent}{val}" for val in reversed(y_hex)])

    # Return the fully formatted G1Affine string
    return f"""G1Affine::new(
{base_indent}BaseField::new(BigInt::new([
{x_formatted}
{base_indent}])),
{base_indent}BaseField::new(BigInt::new([
{y_formatted}
{base_indent}])),
{base_indent.replace("    ", "")})"""

def format_g2(data, base_indent="            "):
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
    indent = base_indent + "        "

    # Reverse and format each list of hex strings
    x_c0_formatted = ",\n".join([f"{indent}{val}" for val in reversed(x_c0_hex)])
    x_c1_formatted = ",\n".join([f"{indent}{val}" for val in reversed(x_c1_hex)])
    y_c0_formatted = ",\n".join([f"{indent}{val}" for val in reversed(y_c0_hex)])
    y_c1_formatted = ",\n".join([f"{indent}{val}" for val in reversed(y_c1_hex)])

    # Return the fully formatted G2Affine string
    return f"""G2Affine::new(
{base_indent}Fp2::new(
{base_indent}    BaseField::new(BigInt::new([
{x_c0_formatted}
{base_indent}    ])),
{base_indent}    BaseField::new(BigInt::new([
{x_c1_formatted}
{base_indent}    ])),
{base_indent}),
{base_indent}Fp2::new(
{base_indent}    BaseField::new(BigInt::new([
{y_c0_formatted}
{base_indent}    ])),
{base_indent}    BaseField::new(BigInt::new([
{y_c1_formatted}
{base_indent}    ])),
{base_indent}),
{base_indent.replace("    ", "")})"""


# --- Main execution block ---

# Input JSON data for the proof
input_data = {"pi_a":["12956428466534567284497061765470918711597636993532261526060777302615563290821","16787843101046360798473021107796925039449701604904750601987472460691853088973","1"],"pi_b":[["953176457444488792263507721566066690294526668727543820177630962540111813890","15451196106987584722158772256846169991736381766087284271188588263700149707756"],["23333114385641920496990954276942953046381965902424623329792249936568088769","1345271580871400635693029469569061683499257249120425032842719847112275445689"],["1","0"]],"pi_c":["14792798417949616889254476086014840336290279776679940139626899793046278342272","1234794697714344249525815095934440957523936130962818408253008340198352386389","1"],"protocol":"groth16","curve":"bn128"}

# Start building the Rust code string
rust_code = "Proof::<Bn254> {\n"

# Process pi_a
if 'pi_a' in input_data:
    rust_code += f"    a: {format_g1(input_data['pi_a'])},\n"

# Process pi_b
if 'pi_b' in input_data:
    rust_code += f"    b: {format_g2(input_data['pi_b'])},\n"

# Process pi_c
if 'pi_c' in input_data:
    rust_code += f"    c: {format_g1(input_data['pi_c'])},\n"

rust_code += "}"

# Print the final Rust code to the console
print(rust_code)
