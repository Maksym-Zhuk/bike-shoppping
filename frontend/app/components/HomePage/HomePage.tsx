import React from "react";
import { View, Text, TouchableOpacity, Image } from "react-native";
import Navigation from "../Navigation";
import SearchBar from "./SearchBar";
import BannerShape from "./BannerShape";

export default function HomePage() {
    return (
        <View className="flex-1">
            <Image
                source={require("../../../assets/images/BG.png")}
                className="absolute w-full h-full right-0 bottom-[-145px] z-0"
                resizeMode="cover"
            />

            <View className="px-5 pt-4 z-20">
                <SearchBar />
            </View>

            <BannerShape className="-mt-5">
                <View className="items-center">
                </View>
            </BannerShape>

            <Navigation />
        </View>
    );
}
