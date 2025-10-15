import { View, TouchableOpacity, Animated } from "react-native";
import { LinearGradient } from "expo-linear-gradient";
import { BlurView } from "expo-blur";
import { useRef, useState, useEffect } from "react";
import { Bike, Map, ShoppingCart, User, FileText } from "lucide-react-native";

export default function Navigation() {
    const [activeIndex, setActiveIndex] = useState(0);

    const animations = useRef(
        Array(5)
            .fill(0)
            .map((_, i) => new Animated.Value(i === 0 ? -35 : -10))
    ).current;

    const icons = [
        <Bike color="rgba(255,255,255,0.6)" size={28} />,
        <Map color="rgba(255,255,255,0.6)" size={28} />,
        <ShoppingCart color="rgba(255,255,255,0.6)" size={28} />,
        <User color="rgba(255,255,255,0.6)" size={28} />,
        <FileText color="rgba(255,255,255,0.6)" size={28} />,
    ];

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
        <View className="absolute bottom-0 w-full">
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
                {icons.map((icon, index) => {
                    const isActive = index === activeIndex;
                    return (
                        <TouchableOpacity
                            key={index}
                            activeOpacity={0.8}
                            onPress={() => setActiveIndex(index)}
                        >
                            <Animated.View
                                className="w-[55px] h-[42px] rounded-lg overflow-hidden items-center justify-center"
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
                                    <LinearGradient
                                        colors={["#34C8E8", "#4E4AF2"]}
                                        start={{ x: 0, y: 0 }}
                                        end={{ x: 0, y: 1 }}
                                        style={{
                                            flex: 1,
                                            justifyContent: "center",
                                            alignItems: "center",
                                            borderRadius: 8,
                                            transform: [{ skewX: "20deg" }],
                                            shadowColor: "#000",
                                            shadowOpacity: 0.25,
                                            shadowOffset: { width: 0, height: 4 },
                                            shadowRadius: 8,
                                            elevation: 8,
                                            width: 80,
                                        }}
                                    >
                                        {icon}
                                    </LinearGradient>
                                ) : (
                                    <View
                                        className="flex-1 justify-center items-center"
                                        style={{
                                            transform: [{ skewX: "20deg" }],
                                            opacity: 0.8,
                                        }}
                                    >
                                        {icon}
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
