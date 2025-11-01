import { View, Text, Image, TouchableOpacity } from "react-native";
import { Trash2 } from "lucide-react-native";
import React from "react";
import { LinearGradient } from "expo-linear-gradient";
interface CartItemProps {
    product: {
        _id: string;
        name: string;
        price: number;
        images: string[];
    };
    onRemove: (id: string) => void;
}

export default function CartItem({ product, onRemove }: CartItemProps) {
    return (
        <View className="w-full flex-row items-center p-4 mb-4 border-b border-[#353F54]">
            <LinearGradient
                colors={["#48526A", "#363E51"]}
                start={{ x: 0, y: 0 }}
                end={{ x: 1, y: 1 }}
                style={{
                    width: 110,
                    height: 80,
                    borderRadius: 16,
                    alignItems: "center",
                    justifyContent: "center",
                    overflow: "hidden",
                }}
            >
                <Image
                    source={{ uri: product.images[0] }}
                    style={{
                        width: 95,
                        height: 60,
                        borderRadius: 20,
                    }}
                    resizeMode="cover"
                />
            </LinearGradient>

            <View style={{ flex: 1, marginLeft: 16 }}>
                <Text className="text-white font-semibold text-lg">{product.name}</Text>
                <Text className="text-[#3C9EEA] opacity-70 mt-1">$ {product.price}</Text>
            </View>

            <TouchableOpacity onPress={() => onRemove(product._id)}>
                <Trash2 size={22} color="#E53935" />
            </TouchableOpacity>
        </View>


    );
}
