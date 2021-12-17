use to_binary::BinaryString;

// Type ID 0 : then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
// Type ID 1 : then the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
// Type ID 4 : literal value
// Type ID 6 : operator

#[derive(PartialEq, Eq, Debug)]
enum PacketType {
    Literal,
    Operator(u8),
    Err,
}

impl PacketType {
    fn from_str(input: &str) -> Self {
        match input {
            "100" => Self::Literal,
            _ => Self::Operator(u8::from_str_radix(input, 2).unwrap()),
        }
    }
}

impl Default for PacketType {
    fn default() -> Self {
        PacketType::Err
    }
}

#[derive(PartialEq, Eq, Debug, Default)]
struct Packet {
    version: u32,
    packet_type: PacketType,
    literal_value: usize,
    operations_packets: Vec<Packet>,
}

fn parse_packet(input: &str) -> (Packet, &str) {
    let version = u32::from_str_radix(&input[0..3], 2).unwrap();

    match PacketType::from_str(&input[3..6]) {
        PacketType::Literal => {
            let mut start_num: usize = 6;
            let mut binary_rep: Vec<&str> = Vec::new();

            while "1" == &input[start_num..start_num + 1] {
                binary_rep.push(&input[(start_num + 1)..(start_num + 5)]);
                start_num += 5;
            }

            binary_rep.push(&input[(start_num + 1)..(start_num + 5)]);

            //binary_rep = format!("{}{}", binary_rep, &input[(start_num + 1)..(start_num + 5)]);

            // println!(
            //     "LITERAL : {}",
            //     usize::from_str_radix(&binary_rep, 2).unwrap()
            // );
            // println!("BIN : {}", binary_rep);

            (
                Packet {
                    version,
                    packet_type: PacketType::Literal,
                    literal_value: usize::from_str_radix(&binary_rep.join(""), 2).unwrap(),
                    operations_packets: Vec::new(),
                },
                &input[(start_num + 5)..],
            )
        }

        PacketType::Operator(op_type) => {
            let i_size_type: usize = 6;

            match &input.chars().nth(i_size_type) {
                Some('0') => {
                    let mut sub_packets: Vec<Packet> = Vec::new();
                    let _length = usize::from_str_radix(&input[7..22], 2).unwrap();

                    let mut sub_input = &input[22..];

                    while sub_input.len() >= 11 {
                        let (pack, remainder) = parse_packet(sub_input);
                        sub_packets.push(pack);
                        sub_input = remainder;
                    }

                    (
                        Packet {
                            version,
                            packet_type: PacketType::Operator(op_type),
                            literal_value: 0,
                            operations_packets: sub_packets,
                        },
                        sub_input,
                    )
                }
                _ => {
                    let num_pack = usize::from_str_radix(&input[7..18], 2).unwrap();

                    let mut sub_packets: Vec<Packet> = Vec::new();

                    let mut sub_input = &input[18..];

                    for _ in 0..num_pack {
                        if sub_input.len() >= 11 {
                            let (pack, remainder) = parse_packet(sub_input);
                            sub_packets.push(pack);
                            sub_input = remainder;
                        }
                    }

                    (
                        Packet {
                            version,
                            packet_type: PacketType::Operator(op_type),
                            literal_value: 0,
                            operations_packets: sub_packets,
                        },
                        sub_input,
                    )
                }
            }
        }
        _ => (
            Packet {
                version: 0,
                packet_type: PacketType::Err,
                literal_value: 0,
                operations_packets: Vec::new(),
            },
            input,
        ),
    }
}

fn compute_version_num(packet: &Packet) -> u32 {
    match packet.packet_type {
        PacketType::Literal => packet.version,
        PacketType::Operator(_) => {
            packet.version
                + &packet
                    .operations_packets
                    .iter()
                    .map(compute_version_num)
                    .sum::<u32>()
        }
        PacketType::Err => 0,
    }
}

fn compute_packet(packet: &Packet) -> usize {
    match packet.packet_type {
        PacketType::Literal => packet.literal_value,
        PacketType::Operator(0) => packet
            .operations_packets
            .iter()
            .fold(0, |acc, el| acc + compute_packet(el)), //Sum
        PacketType::Operator(1) => {
            //let sub_packets: &Vec<Packet> = &packet.operations_packets.as_ref().unwrap();
            packet
                .operations_packets
                .iter()
                .fold(1, |acc, el| acc * compute_packet(el))
        }
        PacketType::Operator(2) => {
            //let sub_packets = &mut packet.operations_packets.as_ref().unwrap().iter();
            packet.operations_packets.iter().fold(usize::MAX, |r, p| {
                let res = compute_packet(p);
                if r < res {
                    r
                } else {
                    res
                }
            })
        }
        PacketType::Operator(3) => {
            //let sub_packets = &mut packet.operations_packets.as_ref().unwrap().iter();
            packet.operations_packets.iter().fold(0, |r, p| {
                let res = compute_packet(p);
                if r > res {
                    r
                } else {
                    res
                }
            })
        }
        PacketType::Operator(5) => {
            let sub_packets = &mut packet.operations_packets.iter();
            if compute_packet(sub_packets.next().unwrap())
                > compute_packet(sub_packets.next().unwrap())
            {
                1
            } else {
                0
            }
        }
        PacketType::Operator(6) => {
            let sub_packets = &mut packet.operations_packets.iter();
            if compute_packet(sub_packets.next().unwrap())
                < compute_packet(sub_packets.next().unwrap())
            {
                1
            } else {
                0
            }
        }
        PacketType::Operator(7) => {
            let sub_packets = &mut packet.operations_packets.iter();
            if compute_packet(sub_packets.next().unwrap())
                == compute_packet(sub_packets.next().unwrap())
            {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn main() {
    let conv = BinaryString::from_hex("020D74FCE27E600A78020200DC298F1070401C8EF1F21A4D6394F9F48F4C1C00E3003500C74602F0080B1720298C400B7002540095003DC00F601B98806351003D004F66011148039450025C00B2007024717AFB5FBC11A7E73AF60F660094E5793A4E811C0123CECED79104ECED791380069D2522B96A53A81286B18263F75A300526246F60094A6651429ADB3B0068937BCF31A009ADB4C289C9C66526014CB33CB81CB3649B849911803B2EB1327F3CFC60094B01CBB4B80351E66E26B2DD0530070401C82D182080803D1C627C330004320C43789C40192D002F93566A9AFE5967372B378001F525DDDCF0C010A00D440010E84D10A2D0803D1761045C9EA9D9802FE00ACF1448844E9C30078723101912594FEE9C9A548D57A5B8B04012F6002092845284D3301A8951C8C008973D30046136001B705A79BD400B9ECCFD30E3004E62BD56B004E465D911C8CBB2258B06009D802C00087C628C71C4001088C113E27C6B10064C01E86F042181002131EE26C5D20043E34C798246009E80293F9E530052A4910A7E87240195CC7C6340129A967EF9352CFDF0802059210972C977094281007664E206CD57292201349AA4943554D91C9CCBADB80232C6927DE5E92D7A10463005A4657D4597002BC9AF51A24A54B7B33A73E2CE005CBFB3B4A30052801F69DB4B08F3B6961024AD4B43E6B319AA020020F15E4B46E40282CCDBF8CA56802600084C788CB088401A8911C20ECC436C2401CED0048325CC7A7F8CAA912AC72B7024007F24B1F789C0F9EC8810090D801AB8803D11E34C3B00043E27C6989B2C52A01348E24B53531291C4FF4884C9C2C10401B8C9D2D875A0072E6FB75E92AC205CA0154CE7398FB0053DAC3F43295519C9AE080250E657410600BC9EAD9CA56001BF3CEF07A5194C013E00542462332DA4295680")
            .unwrap()
            .to_string();

    let (pack, _remainder) = parse_packet(&conv);

    println!("PART1 : {}", compute_version_num(&pack));
    println!("PART2 : {}", usize::MAX - compute_packet(&pack));
}

mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let _input = "38006F45291200";

        let conv = BinaryString::from_hex("38006F45291200")
            .unwrap()
            .to_string();

        assert_eq!(
            String::from("00111000000000000110111101000101001010010001001000000000"),
            conv
        );
    }

    #[test]
    fn test_parse_first_exemple() {
        let input = "110100101111111000101000";

        let expected = Packet {
            version: 6,
            packet_type: PacketType::Literal,
            literal_value: 2021,
            operations_packets: Vec::new(),
        };

        assert_eq!((expected, "000"), parse_packet(input))
    }

    #[test]
    fn test_parse_second_example() {
        let _input = "38006F45291200";

        let conv = BinaryString::from_hex("38006F45291200")
            .unwrap()
            .to_string();

        let (_pack, _remainder) = parse_packet(&conv);

        assert_eq!(1, 1);
    }

    #[test]
    fn test_parse_third_example() {
        let input = "11101110000000001101010000001100100000100011000001100000";

        let (_pack, _remainder) = parse_packet(input);

        assert_eq!(1, 1);
    }

    #[test]
    fn test_sum_second_example() {
        let conv = BinaryString::from_hex("8A004A801A8002F478")
            .unwrap()
            .to_string();

        let (pack, _remainder) = parse_packet(&conv);

        assert_eq!(16, compute_version_num(&pack));
    }

    #[test]
    fn test_sum_third_example() {
        let conv = BinaryString::from_hex("620080001611562C8802118E34")
            .unwrap()
            .to_string();

        let (pack, _remainder) = parse_packet(&conv);

        assert_eq!(12, compute_version_num(&pack));
    }

    #[test]
    fn test_sum_fourth_example() {
        let conv = BinaryString::from_hex("A0016C880162017C3686B18A3D4780")
            .unwrap()
            .to_string();

        let (pack, _remainder) = parse_packet(&conv);

        assert_eq!(31, compute_version_num(&pack));
    }

    #[test]
    fn test_sum_fifth_example() {
        let conv = BinaryString::from_hex("C0015000016115A2E0802F182340")
            .unwrap()
            .to_string();

        let (pack, _remainder) = parse_packet(&conv);

        assert_eq!(23, compute_version_num(&pack));
    }

    #[test]
    fn test_compute1() {
        let conv = BinaryString::from_hex("C200B40A82").unwrap().to_string();
        let (pack, _) = parse_packet(&conv);
        println!("{:?}", pack);
        assert_eq!(3, compute_packet(&pack))
    }

    #[test]
    fn test_compute2() {
        let conv = BinaryString::from_hex("04005AC33890").unwrap().to_string();
        let (pack, _) = parse_packet(&conv);
        println!("{:?}", pack);
        assert_eq!(54, compute_packet(&pack))
    }

    #[test]
    fn test_compute3() {
        let conv = BinaryString::from_hex("880086C3E88112")
            .unwrap()
            .to_string();
        let (pack, _) = parse_packet(&conv);
        println!("{:?}", pack);
        assert_eq!(7, compute_packet(&pack))
    }

    #[test]
    fn test_compute4() {
        let conv = BinaryString::from_hex("CE00C43D881120")
            .unwrap()
            .to_string();
        let (pack, _) = parse_packet(&conv);
        println!("{:?}", pack);
        assert_eq!(9, compute_packet(&pack))
    }

    #[test]
    fn test_compute5() {
        let conv = BinaryString::from_hex("D8005AC2A8F0").unwrap().to_string();
        let (pack, _) = parse_packet(&conv);
        println!("{:?}", pack);
        assert_eq!(1, compute_packet(&pack))
    }

    #[test]
    fn test_compute6() {
        let conv = BinaryString::from_hex("F600BC2D8F").unwrap().to_string();
        let (pack, _) = parse_packet(&conv);
        println!("{:?}", pack);
        assert_eq!(0, compute_packet(&pack))
    }

    #[test]
    fn test_compute7() {
        let conv = BinaryString::from_hex("9C005AC2F8F0").unwrap().to_string();
        let (pack, _) = parse_packet(&conv);
        println!("{:?}", pack);
        assert_eq!(0, compute_packet(&pack))
    }

    #[test]
    fn test_compute8() {
        let conv = BinaryString::from_hex("9C0141080250320F1802104A08")
            .unwrap()
            .to_string();
        let (pack, _) = parse_packet(&conv);
        println!("{:?}", pack);
        assert_eq!(1, compute_packet(&pack))
    }
}
