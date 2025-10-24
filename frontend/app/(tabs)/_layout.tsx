import { Tabs } from "expo-router";import React, { useRef, useEffect, ReactNode } from "react";
import "../global.css";
import { View, TouchableOpacity, Animated } from "react-native";
import { LinearGradient } from "expo-linear-gradient";
import { BlurView } from "expo-blur";
import { Bike, Map, ShoppingCart, User, FileText } from "lucide-react-native";
import Svg, { Path, Defs, LinearGradient as SvgGradient, Stop, G } from "react-native-svg";
import { useRouter, usePathname } from 'expo-router';
import type { Href } from "expo-router";
interface IconShapeProps {
    children: ReactNode;
}

const IconShape: React.FC<IconShapeProps> = ({ children }) => {
    return (
        <View className="items-center justify-center">
            <Svg width={100} height={120} viewBox="0 0 102 100" fill="none">
                <Defs>
                    <SvgGradient
                        id="paint0_linear"
                        x1="0"
                        y1="0"
                        x2="107"
                        y2="103"
                        gradientUnits="userSpaceOnUse"
                    >
                        <Stop offset="0" stopColor="#34C8E8" />
                        <Stop offset="1" stopColor="#4E4AF2" />
                    </SvgGradient>

                    <SvgGradient
                        id="paint1_linear"
                        x1="0"
                        y1="0"
                        x2="107"
                        y2="103"
                        gradientUnits="userSpaceOnUse"
                    >
                        <Stop offset="0" stopColor="white" />
                        <Stop offset="1" stopColor="black" />
                    </SvgGradient>
                </Defs>

                <G>
                    <Path
                        d="M17 20.198C17 15.4312 20.3646 11.3271 25.0388 10.3922L65.0388 2.39223C71.2268 1.15465 77 5.88758 77 12.198V39.802C77 44.5688 73.6354 48.6729 68.9612 49.6078L28.9612 57.6078C22.7732 58.8454 17 54.1124 17 47.802V20.198Z"
                        fill="url(#paint0_linear)"
                    />
                    <Path
                        d="M65.1367 2.88281C71.0152 1.70711 76.5 6.2033 76.5 12.1982V39.8018C76.5 44.3302 73.3037 48.229 68.8633 49.1172L28.8633 57.1172C22.9848 58.2929 17.5 53.7967 17.5 47.8018V20.1982C17.5 15.6698 20.6963 11.771 25.1367 10.8828L65.1367 2.88281Z"
                        stroke="url(#paint1_linear)"
                        strokeOpacity={0.6}
                    />
                </G>
            </Svg>

            <View
                style={{
                    position: "absolute",
                    top: -40,
                    left: -10,
                    right: 0,
                    bottom: 0,
                    alignItems: "center",
                    justifyContent: "center",
                }}
            >
                {children}
            </View>
        </View>
    );
};
export function Navigation() {
    const router = useRouter();
    const pathname = usePathname();

    const animations = useRef<Animated.Value[]>(
        Array(5)
            .fill(0)
            .map((_, i) => new Animated.Value(i === 0 ? -35 : -10))
    ).current;

    const icons: { icon: React.ReactNode; route: Href }[] = [
        { icon: <Bike color="rgba(255,255,255,0.6)" size={28} />, route: "/" },
        { icon: <Map color="rgba(255,255,255,0.6)" size={28} />, route: "/map" },
        { icon: <ShoppingCart color="rgba(255,255,255,0.6)" size={28} />, route: "/shop" },
        { icon: <User color="rgba(255,255,255,0.6)" size={28} />, route: "/user" },
        { icon: <FileText color="rgba(255,255,255,0.6)" size={28} key="file" />, route: "/documentation" },
    ];

    const activeIndex = icons.findIndex(item => item.route === pathname);

    useEffect(() => {
        animations.forEach((anim, i) => {
            Animated.spring(anim, {
                toValue: i === activeIndex ? -35 : -10,
                useNativeDriver: true,
                speed: 12,
                bounciness: 8,
            }).start();
        });
    }, [activeIndex]);

    return (
        <View className="absolute bottom-8 w-full">
            <LinearGradient
                colors={["#363E51", "#181C24"]}
                style={{
                    position: "absolute",
                    top: -25,
                    left: 0,
                    right: 0,
                    bottom: -35,
                    opacity: 0.5,
                }}
            />
            <BlurView
                intensity={40}
                tint="dark"
                style={{
                    position: "absolute",
                    top: -25,
                    left: 0,
                    right: 0,
                    bottom: -30,
                }}
            />

            <View className="w-full h-full px-9 flex flex-row items-center justify-between">
                {icons.map((item, index) => {
                    const isActive = index === activeIndex;
                    return (
                        <TouchableOpacity
                            key={index}
                            activeOpacity={0.8}
                            onPress={() => router.push(item.route)}

                        >
                            <Animated.View
                                className="w-[55px] h-[42px] rounded-lg overflow-visible items-center justify-center"
                                style={{
                                    zIndex: isActive ? 5 : 1,
                                    transform: [
                                        { skewX: "-20deg" },
                                        { translateY: animations[index] },
                                        { scale: isActive ? 1.05 : 1 },
                                    ],
                                }}
                            >
                                {isActive ? (
                                    <View
                                        style={{
                                            transform: [{ skewX: "20deg" },],
                                            shadowColor: "#000",
                                            shadowOpacity: 0.25,
                                            shadowOffset: { width: 0, height: 4 },
                                            shadowRadius: 8,
                                            elevation: 8,
                                        }}
                                    >
                                        <IconShape>{item.icon}</IconShape>
                                    </View>
                                ) : (
                                    <View
                                        className="flex-1 justify-center items-center"
                                        style={{
                                            transform: [{ skewX: "20deg" }],
                                            opacity: 0.8,
                                        }}
                                    >
                                        {item.icon}
                                    </View>
                                )}
                            </Animated.View>
                        </TouchableOpacity>
                    );
                })}
            </View>
        </View>
    );
}

export default function RootLayout() {
  return (
    <Tabs
      screenOptions={{
        headerShown: false,
        animation: 'fade',
        tabBarStyle: {
          backgroundColor: '#242C38',
        }
      }}
      tabBar={()=>{
        return <Navigation/>
      }}
      >
    </Tabs>
  )
}
