import { View } from "react-native";
import Product from "./Product";
import { useState, useEffect } from "react";
import axios from "axios";
import AsyncStorage from '@react-native-async-storage/async-storage';

export default function Products() {
    const [products, setProducts] = useState([]);
    const [shoppingCart, setShoppingCart] = useState<string[]>([]);

    useEffect(() => {
        axios.get('http://192.168.0.113:8080/api/product/products')
            .then(res => setProducts(res.data));
        // Load existing cart from AsyncStorage
        AsyncStorage.getItem('shoppingCart').then(data => {
            if (data) setShoppingCart(JSON.parse(data));
        });
    }, []);

    const saveShoppingCart = async (itemId: string) => {
        let updatedCart = [...shoppingCart];

        if (shoppingCart.includes(itemId)) {
            updatedCart = updatedCart.filter(id => id !== itemId);
        } else {
            updatedCart.push(itemId);
        }

        setShoppingCart(updatedCart);
        await AsyncStorage.setItem('shoppingCart', JSON.stringify(updatedCart));
        console.log("Updated cart:", updatedCart);
    };


    return (
        <View className="w-full flex-row flex-wrap gap-x-7 px-6 -mt-6">
            {products.map((product, index) => (
                <Product
                    key={index}
                    content={product}
                    index={index}
                    shoppingCart={shoppingCart}
                    onSaveShoppingCart={saveShoppingCart}
                />

            ))}
        </View>
    );
}