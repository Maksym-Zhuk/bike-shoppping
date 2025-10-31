import React from "react";
import { View, Text, Image } from "react-native";
import Svg, { Defs, LinearGradient, Stop, Path, G } from "react-native-svg";

interface ProductProps {
    content?: {
        name: string;
        price: number;
        description: string;
        images: string[];
        discount: number;
        category: string;
    };
    index?: number;
}

export default function Product({ content, index }: ProductProps) {
    if (!content) return null;

    return (
        <View
            className={`items-center justify-center w-[180px] ${(index ?? 0) % 2 === 1 ? "-mt-10" : ""} ${((index ?? 0) > 1) ? "-mt-10" : ""}`}
            style={{ height: 300 }}
        >
            <Svg
                width={185}
                height={260}
                viewBox="0 0 165 234"
                fill="none"
                style={{
                    position: "absolute",
                    bottom: 0,
                }}
            >
                <Defs>
                    <LinearGradient
                        id="paint0_linear"
                        x1="46"
                        y1="58.0679"
                        x2="82.8287"
                        y2="170.834"
                        gradientUnits="userSpaceOnUse"
                    >
                        <Stop stopColor="#363E51" />
                        <Stop offset="1" stopColor="#191E26" />
                    </LinearGradient>

                    <LinearGradient
                        id="paint1_linear"
                        x1="17.5"
                        y1="3.20894"
                        x2="135.882"
                        y2="61.3003"
                        gradientUnits="userSpaceOnUse"
                    >
                        <Stop stopColor="white" />
                        <Stop offset="1" stopOpacity="0" />
                    </LinearGradient>
                </Defs>

                <G>
                    <Path
                        d="M0 39.7015C0 29.8576 7.16305 21.4775 16.8869 19.9452L141.887 0.248262C154.024 -1.66429 165 7.71737 165 20.0045V190.915C165 200.58 158.088 208.863 148.578 210.592L23.5777 233.319C11.3009 235.551 0 226.12 0 213.642V39.7015Z"
                        fill="url(#paint0_linear)"
                        fillOpacity={0.6}
                    />
                    <Path
                        d="M142.043 1.23608C153.573 -0.580586 164 8.332 164 20.0046V190.915C164 200.097 157.433 207.966 148.398 209.608L23.3984 232.336C11.7358 234.456 1.00023 225.496 1 213.642V39.7019C1 30.3503 7.80531 22.389 17.043 20.9333L142.043 1.23608Z"
                        stroke="url(#paint1_linear)"
                        strokeOpacity={0.2}
                        strokeWidth={2}
                    />
                </G>
            </Svg>

            <View className="absolute inset-0 items-center justify-start pt-12">
                <Image
                    source={{ uri: content.images[0] }}
                    className="w-[90%] h-[170px] z-10"
                    resizeMode="contain"
                />
                <Text className="absolute text-[rgba(255,255,255,0.6)] text-[18px] font-medium top-[185px] left-[15px]">
                    {(content.category == "1") ? "Bikes" : "Accessories"}
                </Text>
                <Text className="absolute text-[rgba(255,255,255,0.7)] text-[18px] font-extrabold top-[207px] left-[15px] pr-2">
                    {content.name}
                </Text>
                <Text className="absolute text-[rgba(255,255,255,0.6)] text-[18px] font-medium top-[250px] left-[15px]">
                    $ {content.price}
                </Text>
            </View>
        </View >
    );
}
