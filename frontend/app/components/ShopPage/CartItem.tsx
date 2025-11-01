import { View, Text, Image, TouchableOpacity, Alert } from "react-native";
import React, { useState } from "react";
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
    const [quantity, setQuantity] = useState(1);

    const handleDecrease = () => {
        if (quantity > 1) {
            setQuantity(prev => prev - 1);
        } else {
            Alert.alert(
                "Remove product",
                "Do you want to remove this product from the cart?",
                [
                    { text: "Cancel", style: "cancel" },
                    { text: "Remove", style: "destructive", onPress: () => onRemove(product._id) },
                ]
            );
        }
    };

    const handleIncrease = () => {
        setQuantity(prev => prev + 1);
    };

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

            <View className="flex-row items-center bg-[#1b212d]">
                <TouchableOpacity
                    className="px-3 py-1 border bg-[#34C8E8] rounded-md"
                    onPress={handleIncrease}
                >
                    <Text className="text-white text-lg">+</Text>
                </TouchableOpacity>
                <View className="px-4 py-1 border-t border-b">
                    <Text className="text-white text-lg">{quantity}</Text>
                </View>
                <TouchableOpacity
                    className="px-3 py-1 border bg-[#353F54] rounded-md"
                    onPress={handleDecrease}
                >
                    <Text className="text-white text-lg">-</Text>
                </TouchableOpacity>

            </View>
        </View>
    );
}
