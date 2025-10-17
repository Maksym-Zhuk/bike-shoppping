import React from "react";
import { View, Dimensions } from "react-native";
import Svg, { Defs, LinearGradient, Stop, Path } from "react-native-svg";
import { BlurView } from "expo-blur";
import MaskedView from "@react-native-masked-view/masked-view";

interface BannerShapeProps {
    children?: React.ReactNode;
    className?: string;
}

const { width: SCREEN_WIDTH } = Dimensions.get("window");
const ORIGINAL_W = 390;
const ORIGINAL_H = 358;
const BANNER_HEIGHT = Math.round((SCREEN_WIDTH * ORIGINAL_H) / ORIGINAL_W);

export default function BannerShape({ children, className }: BannerShapeProps) {
    const pathD =
        "M20 60C20 48.9543 28.9543 40 40 40H350C361.046 40 370 48.9543 370 60V222.156C370 232.323 362.372 240.872 352.271 242.026L42.2709 277.455C30.4029 278.811 20 269.529 20 257.584V60Z";

    return (
        <View
            className={`relative max-h-[345px] overflow-hidden z-10 ${className}`}
            style={{ width: SCREEN_WIDTH, height: BANNER_HEIGHT }}
        >
            <Svg
                width={SCREEN_WIDTH}
                height={BANNER_HEIGHT}
                viewBox={`0 0 ${ORIGINAL_W} ${ORIGINAL_H}`}
                preserveAspectRatio="xMidYMid meet"
                className="absolute inset-0"
            >
                <Defs>
                    <LinearGradient
                        id="paint0"
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
                        id="paint1"
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

                <Path
                    d={pathD}
                    fill="url(#paint0)"
                    fillOpacity={0.6}
                    stroke="url(#paint1)"
                    strokeWidth={2}
                    strokeOpacity={0.2}
                />
            </Svg>

            <MaskedView
                style={{ position: "absolute", top: 0, left: 0, right: 0, bottom: 0 }}
                maskElement={
                    <Svg
                        width={SCREEN_WIDTH}
                        height={BANNER_HEIGHT}
                        viewBox={`0 0 ${ORIGINAL_W} ${ORIGINAL_H}`}
                    >
                        <Path d={pathD} fill="white" />
                    </Svg>
                }
            >
                <BlurView intensity={40} tint="dark" className="flex-1" />

                <View className="absolute inset-0 bg-white/10" />
            </MaskedView>

            <View className="absolute inset-0 items-center justify-center px-6">
                {children}
            </View>
        </View>
    );
}
