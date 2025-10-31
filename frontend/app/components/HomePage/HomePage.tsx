import React from "react";
import { View, Text, Image, Pressable, ScrollView } from "react-native";
import SearchBar from "./SearchBar";
import BannerShape from "./BannerShape";
import AppIcons from "./AppIcons";
import Products from "./Products";
import { useRouter } from "expo-router";

export default function HomePage() {
  const router = useRouter();

  return (
    <View className="flex-1 bg-[#242C38]">
      <Image
        source={require("../../../assets/images/BG.png")}
        className="absolute w-full h-full right-0 bottom-[-145px] z-0"
        resizeMode="cover"
      />

      <ScrollView
        className="flex-1"
        contentContainerStyle={{
          paddingBottom: 60,
        }}
        showsVerticalScrollIndicator={false}
      >
        <View className="px-5 pt-4 z-20">
          <SearchBar />
        </View>

        <BannerShape className="-mt-5">
          <Pressable
            onPress={() =>
              router.push({
                pathname: "/product",
                params: { a: "1" },
              })
            }
            className="w-full items-center justify-center relative"
          >
            <Image
              source={require("../../../assets/images/banner-bike-sample.png")}
              className="w-[90%] h-[170px] z-10"
              resizeMode="contain"
            />
            <Text className="absolute text-[rgba(255,255,255,0.7)] text-[32px] font-bold top-[175px] left-[25px]">
              30% Off
            </Text>
          </Pressable>
        </BannerShape>

        <AppIcons />
        <Products />
      </ScrollView>
    </View>
  );
}
