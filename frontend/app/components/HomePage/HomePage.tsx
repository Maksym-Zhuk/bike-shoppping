import { Text, View, Image } from "react-native";
import Navigation from "../Navigation";
import SearchBar from "./SearchBar";
export default function HomePage() {
    return (
        <View className="w-full h-full">
            <View className="px-5 pt-4">
                <SearchBar />
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
