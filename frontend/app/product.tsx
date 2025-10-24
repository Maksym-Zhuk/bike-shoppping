import {useLocalSearchParams} from "expo-router"
import {Text} from "react-native";
export default function Product(){
  var params = useLocalSearchParams();
  return (<Text>{params.a}</Text>)
}
