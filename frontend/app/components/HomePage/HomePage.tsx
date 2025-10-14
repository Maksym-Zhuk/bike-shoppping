import { Text, View, Image } from "react-native";

export default function HomePage() {
    return (
        <View className="w-full h-full">
            <Image
                source={require("../../../assets/images/BG.png")}
                className="absolute w-[110%] h-full right-[-12px] bottom-[-35px]"
                resizeMode="cover"
            />
        </View>
    );
}
