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
                <View className="w-full items-center justify-center -mt-[100px] relative">
                    <Image
                        source={require("../../../assets/images/banner-bike-sample.png")}
                        className="w-[90%] h-[170px] z-10"
                        resizeMode="contain"
                    />
                    <Text className="absolute text-[rgba(255,255,255,0.7)] text-[32px] font-bold top-[175px] left-[25px]">30% Off</Text>
                </View>
            </BannerShape>

            <Navigation />
        </View>
    );
}
