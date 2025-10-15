import { View, Text } from "react-native";
import { LinearGradient } from "expo-linear-gradient";
import { BlurView } from "expo-blur";
import { Bike, Map, ShoppingCart, User, FileText } from 'lucide-react-native'
export default function Navigation() {
    const navigationOptions = [
        {
            icon: <Bike color='rgba(255,255,255,0.6)' />
        },
        {
            icon: <Map color='rgba(255,255,255,0.6)' />
        },
        {
            icon: <ShoppingCart color='rgba(255,255,255,0.6)' />
        },
        {
            icon: <User color='rgba(255,255,255,0.6)' />
        },
        {
            icon: <FileText color='rgba(255,255,255,0.6)' />
        },
    ]
    return (
        <View className="absolute bottom-0 w-full">
            <LinearGradient
                colors={["#363E51", "#181C24"]}
                style={{ position: "absolute", top: -35, left: 0, right: 0, bottom: -35, opacity: 0.5 }}
            />
            <BlurView
                intensity={40}
                tint="dark"
                style={{ position: "absolute", top: -35, left: 0, right: 0, bottom: -30 }}
            />
            <View className="w-full h-full px-12 flex flex-row items-center justify-between">
                {navigationOptions.map((option, index) => (
                    <View
                        key={index}
                        style={{ transform: [{ translateY: -10 }] }}
                    >
                        {option.icon}
                    </View>
                ))}
            </View>
        </View>
    );
}
