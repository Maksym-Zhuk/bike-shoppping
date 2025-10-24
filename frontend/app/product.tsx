import React, { useCallback, useRef} from 'react';
import {Image, Pressable}  from "react-native"
import { Text, StyleSheet } from 'react-native';
import { GestureHandlerRootView } from 'react-native-gesture-handler';
import BottomSheet, { BottomSheetView } from '@gorhom/bottom-sheet';

const App = () => {
  // ref
  const bottomSheetRef = useRef<BottomSheet>(null);
  const snapPoints = [100, '50%'];
  // callbacks
  const handleSheetChanges = useCallback((index: number) => {
    console.log('handleSheetChanges', index);
  }, []);

  // renders
  return (
    <GestureHandlerRootView style={styles.container}>

<Image
                source={require("../assets/images/BG.png")}
                className="absolute w-full h-full right-0 bottom-[-145px] z-0"
                resizeMode="cover"
            />
      <BottomSheet
        ref={bottomSheetRef}
        onChange={handleSheetChanges}
        backgroundStyle={{
          backgroundColor: "#353F54"
        }}
        snapPoints={snapPoints}
      >
        <BottomSheetView className="flex-1 flex-row justify-center gap-10">
        <Pressable><Text>Description</Text></Pressable>
        <Pressable><Text>Specification</Text></Pressable>
        </BottomSheetView>
      </BottomSheet>
    </GestureHandlerRootView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#242C38',
  },
  contentContainer: {
    flex: 1,
    padding: 36,
    alignItems: 'center',
  },
});

export default App;
