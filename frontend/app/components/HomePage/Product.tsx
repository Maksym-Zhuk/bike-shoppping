import React from "react";
import { View, Text, Image } from "react-native";
import { Svg, Defs, LinearGradient, Stop, Path } from "react-native-svg";
import { BlurView } from "expo-blur";
import { G } from "react-native-svg";

interface ProductProps {
    content?: {
        name: string;
        price: number;
        description: string;
        images: string[];
        discount: number;
    },
    index?: number;
}

export default function Product({ content, index }: ProductProps) {
    if (!content) return null;

    return (
        <View className={`items-center justify-center w-[180px] ${(index ?? 0) % 2 === 1 ? "-mt-16" : ""}`}>
            <Svg
                width={285}
                height={210}
                viewBox="0 9 90 45"
                fill="none"
            >
                <Defs>
                    <LinearGradient
                        id="paint0_linear"
                        x1="146.212"
                        y1="103.734"
                        x2="164.149"
                        y2="251.851"
                        gradientUnits="userSpaceOnUse"
                    >
                        <Stop stopColor="#353F54" />
                        <Stop offset="1" stopColor="#222834" />
                    </LinearGradient>

                    <LinearGradient
                        id="paint1_linear"
                        x1="59.2424"
                        y1="48.9627"
                        x2="191.249"
                        y2="237.494"
                        gradientUnits="userSpaceOnUse"
                    >
                        <Stop stopColor="white" />
                        <Stop offset="0.844522" stopOpacity="0" />
                        <Stop offset="1" stopOpacity="0" />
                    </LinearGradient>
                </Defs>

                <BlurView intensity={40} tint="dark" className="flex-1" />

                <G>
                    <Path
                        d="M17 20.198C17 15.4312 20.3646 11.3271 25.0388 10.3922L65.0388 2.39223C71.2268 1.15465 77 5.88758 77 12.198V39.802C77 44.5688 73.6354 48.6729 68.9612 49.6078L28.9612 57.6078C22.7732 58.8454 17 54.1124 17 47.802V20.198Z"
                        fill="url(#paint0_linear)"
                        fillOpacity={0.6}
                    />
                    <Path
                        d="M65.1367 2.88281C71.0152 1.70711 76.5 6.2033 76.5 12.1982V39.8018C76.5 44.3302 73.3037 48.229 68.8633 49.1172L28.8633 57.1172C22.9848 58.2929 17.5 53.7967 17.5 47.8018V20.1982C17.5 15.6698 20.6963 11.771 25.1367 10.8828L65.1367 2.88281Z"
                        stroke="url(#paint1_linear)"
                        strokeOpacity={0.2}
                        strokeWidth={0.8}
                    />
                </G>
            </Svg>

            <View
                style={{
                    position: "absolute",
                    top: -50,
                    left: 0,
                    right: 0,
                    bottom: 0,
                    alignItems: "center",
                    justifyContent: "center",
                }}
            >
                {/* {content} */}
            </View>
        </View>
    );
}
