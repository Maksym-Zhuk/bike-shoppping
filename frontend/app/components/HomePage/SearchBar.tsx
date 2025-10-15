import { Text, View } from "react-native";
import { LinearGradient } from "expo-linear-gradient";
import { Search } from "lucide-react-native";

export default function SearchBar() {
    return (
        <View className="w-full flex-row justify-between items-center">
            <Text className="font-bold text-[24px] text-white">Choose your bike</Text>
            <LinearGradient
                colors={["#34C8E8", "#4E4AF2"]}
                start={{ x: 0, y: 0 }}
                end={{ x: 0, y: 1 }}
                style={{
                    width: 43,
                    height: 43,
                    justifyContent: "center",
                    alignItems: "center",
                    borderRadius: 8,
                    shadowColor: "#000",
                    shadowOpacity: 0.25,
                    shadowOffset: { width: 0, height: 4 },
                    shadowRadius: 8,
                }}
            >
                <Search size={24} color="white" />
            </LinearGradient>
        </View>
    );
}
