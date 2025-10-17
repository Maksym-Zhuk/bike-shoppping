import { Text, View } from "react-native";
import { LinearGradient, Stop, Path } from "react-native-svg";
import { BlurView } from "expo-blur";
import React, { ReactElement } from "react";

interface AppIconProps {
    content: ReactElement,
    margin: number,
}
export default function AppIcon({ margin, content }: AppIconProps) {
    return (
        <View
            className={`relative overflow-hidden z-10 w-[60px] h-[60px] rounded-[10px]`}
            style={{ marginTop: -margin }}
        >
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

            <BlurView intensity={40} tint="dark" className="flex-1" />

            <View className="absolute inset-0 bg-white/10" />

            <View className="absolute inset-0 items-center justify-center">
                {content}
            </View>
        </View>
    );
}
