import { View, Text } from "react-native";
import Svg, { Path } from "react-native-svg";
import Product from "./Product";
import { useState, useEffect } from "react";
import axios from "axios";
export default function Products() {
    const [products, setProducts] = useState([]);
    useEffect(() => {
        axios.get('http://192.168.0.113:8080/api/product/products').then(res => setProducts(res.data));
    }, [])

    return (
        <View className="w-full flex-row flex-wrap gap-x-7 px-6 -mt-3">
            {products.map((product, index) => (
                <Product
                    key={index}
                    content={product}
                    index={index}
                />
            ))}
        </View>
    );
}
