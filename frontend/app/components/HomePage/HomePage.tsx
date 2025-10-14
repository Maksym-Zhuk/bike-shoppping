import { Text, View, Image } from "react-native";
import Navigation from "../Navigation";
export default function HomePage() {
    return (
        <View className="w-full h-full">
            <View className="px-5 pt-6">
                {/* content here later on */}
                {/* <Text className="font-bold text-[24px] text-white">Choose your bike</Text> */}
            </View>
            <Image
                source={require("../../../assets/images/BG.png")}
                className="absolute w-[100%] h-full right-0 bottom-[-145px]"
                resizeMode="cover"
            />
            <Navigation />
        </View>
    );
}
