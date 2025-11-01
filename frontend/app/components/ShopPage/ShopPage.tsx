import { View, Text, FlatList, TouchableOpacity } from "react-native";
import { useEffect, useState } from "react";
import axios from "axios";
import AsyncStorage from "@react-native-async-storage/async-storage";
import { LinearGradient } from "expo-linear-gradient";
import CartItem from "./CartItem";

export type ProductType = {
    _id: string;
    name: string;
    price: number;
    description: string;
    images: string[];
    discount: number;
    category: string;
};

export default function ShopPage() {
    const [allProducts, setAllProducts] = useState<ProductType[]>([]);
    const [shoppingCart, setShoppingCart] = useState<string[]>([]);
    const [cartProducts, setCartProducts] = useState<ProductType[]>([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const init = async () => {
            try {
                const stored = await AsyncStorage.getItem("shoppingCart");
                if (stored) setShoppingCart(JSON.parse(stored));

                const response = await axios.get(
                    "http://192.168.0.113:8080/api/product/products"
                );
                setAllProducts(response.data);
            } finally {
                setLoading(false);
            }
        };

        init();
    }, []);

    useEffect(() => {
        if (!loading) {
            const filtered = allProducts.filter((product) =>
                shoppingCart.includes(product._id)
            );
            setCartProducts(filtered);
        }
    }, [allProducts, shoppingCart, loading]);

    //temporary solution to watch cart changes, should be replaced with a global state manager later
    useEffect(() => {
        const interval = setInterval(async () => {
            const stored = await AsyncStorage.getItem("shoppingCart");
            if (stored) setShoppingCart(JSON.parse(stored));
        }, 500);
        return () => clearInterval(interval);
    }, []);


    const handleCheckout = () => {
        alert("Proceeding to checkout!");
    };

    return (
        <View className="flex-1 items-center pt-4">
            <Text className="text-white text-[24px] font-bold mb-4">
                My Shopping Cart
            </Text>

            {loading ? (
                <Text className="text-white mt-8">Loading...</Text>
            ) : cartProducts.length > 0 ? (
                <>
                    <FlatList
                        data={cartProducts}
                        keyExtractor={(item) => item._id}
                        style={{ width: "100%" }}
                        contentContainerStyle={{ paddingHorizontal: 20, paddingBottom: 100 }}
                        renderItem={({ item }) => (
                            <CartItem
                                product={item}
                                onRemove={() => {
                                    const updated = shoppingCart.filter((id) => id !== item._id);
                                    setShoppingCart(updated);
                                    AsyncStorage.setItem("shoppingCart", JSON.stringify(updated));
                                }}
                            />
                        )}
                    />

                    <TouchableOpacity
                        onPress={handleCheckout}
                        style={{
                            position: "absolute",
                            bottom: 120,
                            width: "70%",
                            borderRadius: 16,
                            overflow: "hidden",
                        }}
                    >
                        <LinearGradient
                            colors={["#34C8E8", "#4E4AF2"]}
                            start={{ x: 0, y: 0 }}
                            end={{ x: 1, y: 1 }}
                            style={{
                                paddingVertical: 16,
                                alignItems: "center",
                                borderRadius: 16,
                            }}
                        >
                            <Text className="text-white text-[20px] font-bold">Checkout</Text>
                        </LinearGradient>
                    </TouchableOpacity>
                </>
            ) : (
                <Text className="text-white mt-8">
                    Your cart is empty 😕
                </Text>
            )}
        </View>
    );
}
