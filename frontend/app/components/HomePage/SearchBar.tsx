import {
    Text,
    View,
    TouchableOpacity,
    TextInput,
    Animated,
    Easing,
    TouchableWithoutFeedback,
    Keyboard
} from "react-native";
import { LinearGradient } from "expo-linear-gradient";
import { Search, X } from "lucide-react-native";
import { useRef, useState } from "react";

export default function SearchBar() {
    const [active, setActive] = useState(false);
    const widthAnim = useRef(new Animated.Value(43)).current;
    const fadeHeading = useRef(new Animated.Value(1)).current;
    const inputOpacity = useRef(new Animated.Value(0)).current;

    const toggleSearch = () => {
        Keyboard.dismiss();
        if (!active) {
            Animated.parallel([
                Animated.timing(fadeHeading, {
                    toValue: 0,
                    duration: 250,
                    easing: Easing.out(Easing.ease),
                    useNativeDriver: true,
                }),
                Animated.timing(widthAnim, {
                    toValue: 400,
                    duration: 400,
                    easing: Easing.out(Easing.cubic),
                    useNativeDriver: false,
                }),
                Animated.timing(inputOpacity, {
                    toValue: 1,
                    delay: 250,
                    duration: 300,
                    useNativeDriver: true,
                }),
            ]).start();
        } else {
            Animated.parallel([
                Animated.timing(inputOpacity, {
                    toValue: 0,
                    duration: 150,
                    useNativeDriver: true,
                }),
                Animated.timing(widthAnim, {
                    toValue: 43,
                    duration: 300,
                    easing: Easing.inOut(Easing.cubic),
                    useNativeDriver: false,
                }),
                Animated.timing(fadeHeading, {
                    toValue: 1,
                    delay: 200,
                    duration: 300,
                    useNativeDriver: true,
                }),
            ]).start();
        }
        setActive(!active);
    };

    return (
        <TouchableWithoutFeedback onPress={Keyboard.dismiss}>
            <View className="w-full flex-row justify-between items-center relative">
                <Animated.Text
                    style={{
                        opacity: fadeHeading,
                        transform: [
                            {
                                translateY: fadeHeading.interpolate({
                                    inputRange: [0, 1],
                                    outputRange: [-10, 0],
                                }),
                            },
                        ],
                    }}
                    className="font-bold text-[24px] text-white"
                >
                    Choose your bike
                </Animated.Text>

                <Animated.View
                    style={{
                        width: widthAnim,
                        height: 43,
                        borderRadius: 8,
                        overflow: "hidden",
                        marginLeft: -205,
                    }}
                >
                    <LinearGradient
                        colors={["#34C8E8", "#4E4AF2"]}
                        start={{ x: 0, y: 0 }}
                        end={{ x: 0, y: 1 }}
                        style={{
                            flex: 1,
                            flexDirection: "row",
                            alignItems: "center",
                            paddingHorizontal: 10,
                        }}
                    >
                        <Animated.View
                            style={{
                                flex: 1,
                                opacity: inputOpacity,
                                marginRight: 8,
                            }}
                        >
                            <TextInput
                                placeholder="Search..."
                                placeholderTextColor="#eee"
                                style={{
                                    color: "white",
                                    fontSize: 16,
                                }}
                                onSubmitEditing={Keyboard.dismiss}
                            />
                        </Animated.View>

                        <TouchableOpacity onPress={toggleSearch} className="relative">
                            {active ? (
                                <X size={22} color="white" />
                            ) : (
                                <Search size={24} color="white" style={{ marginLeft: -7 }} />
                            )}
                        </TouchableOpacity>
                    </LinearGradient>
                </Animated.View>
            </View>
        </TouchableWithoutFeedback>
    );
}
