import { View, Text } from "react-native";
import { LinearGradient } from "expo-linear-gradient";
import { BlurView } from "expo-blur";

export default function Navigation() {
    return (
        <View className="absolute bottom-0 w-full">
            <LinearGradient
                colors={["#363E51", "#181C24"]}
                style={{ position: "absolute", top: -13, left: 0, right: 0, bottom: -35, opacity: 0.5 }}
            />
            <BlurView
                intensity={40}
                tint="dark"
                style={{ position: "absolute", top: -13, left: 0, right: 0, bottom: -30 }}
            />
            <Text className="text-white text-center p-4">Hey?</Text>
        </View>
    );
}
