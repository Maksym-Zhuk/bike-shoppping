import { View, Text, FlatList } from "react-native";
import { useEffect, useState } from "react";
import axios from "axios";
import AsyncStorage from "@react-native-async-storage/async-storage";
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

    return (
        <View className="flex-1 items-center pt-4">
            <Text className="text-white text-[24px] font-bold mb-4">
                My Shopping Cart
            </Text>

            {loading ? (
                <Text className="text-white mt-8">Loading...</Text>
            ) : cartProducts.length > 0 ? (
                <FlatList
                    data={cartProducts}
                    keyExtractor={(item) => item._id}
                    style={{ width: "100%" }}
                    contentContainerStyle={{ paddingHorizontal: 20, paddingBottom: 60 }}
                    renderItem={({ item, index }) => (
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

            ) : (
                <Text className="text-white mt-8">
                    Your cart is empty 😕
                </Text>
            )}
        </View>
    );
}
