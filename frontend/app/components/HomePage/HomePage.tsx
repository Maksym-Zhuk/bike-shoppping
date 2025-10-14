import { Text, View, Image } from "react-native";
import Navigation from "../Navigation";
export default function HomePage() {
    return (
        <View className="w-full h-full">
            <Image
                source={require("../../../assets/images/BG.png")}
                className="absolute w-[100%] h-full right-[-20px] bottom-[-45px]"
                resizeMode="cover"
            />
            <Navigation />
        </View>
    );
}
