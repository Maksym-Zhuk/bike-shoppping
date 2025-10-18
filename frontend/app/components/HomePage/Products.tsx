import { View, Text } from "react-native";
import Svg, { Path } from "react-native-svg";
import Product from "./Product";
export default function Products() {
    const placeholderProduct = [
        {
            name: 'Peugeot - LR01 ',
            category: 1,
            images: ["../../../assets/images/banner-bike-sample.png"],
            price: 1.999
        },
        {
            name: 'Peugeot - LR01 ',
            category: 1,
            images: ["../../../assets/images/banner-bike-sample.png"],
            price: 1.999
        },
    ]
    return (
        <View className="w-full flex-row justify-between items-center px-6">
            {placeholderProduct.map((product, index) => (
                <Product key={index} content={product} />
            ))}
        </View>
    );
}
