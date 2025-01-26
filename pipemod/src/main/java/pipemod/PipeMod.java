package pipemod;

import basemod.BaseMod;
import basemod.interfaces.PostInitializeSubscriber;
import basemod.interfaces.StartGameSubscriber;

import com.evacipated.cardcrawl.modthespire.Loader;
import com.evacipated.cardcrawl.modthespire.ModInfo;
import com.evacipated.cardcrawl.modthespire.Patcher;
import com.evacipated.cardcrawl.modthespire.lib.SpireInitializer;

import com.megacrit.cardcrawl.dungeons.AbstractDungeon;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.scannotation.AnnotationDB;

import java.io.*;
import java.nio.charset.StandardCharsets;
import java.util.*;

@SpireInitializer
public class PipeMod implements PostInitializeSubscriber, StartGameSubscriber {
    @Override
    public void receivePostInitialize() {
        logger.info(modID + " Hello, world.");
    }

    @Override
    public void receiveStartGame() {
        int amount = AbstractDungeon.player.gold;
        connectPipe(amount);
    }

    public void connectPipe(int amount) {
        String pipeName = "\\\\.\\pipe\\my-pipe";

        System.out.println("Attempting to connect to the named pipe server...");

        try (RandomAccessFile pipe = new RandomAccessFile(pipeName, "rw")) {
            System.out.println("Connected to the server!");

            // Write a message to the server
            String message = "Hello from the client! " + amount;
            pipe.write(message.getBytes(StandardCharsets.UTF_8));
            System.out.println("Sent to server: " + message);

            // Read response from the server
            byte[] buffer = new byte[1024];
            int bytesRead = pipe.read(buffer);
            String response = new String(buffer, 0, bytesRead, StandardCharsets.UTF_8);
            System.out.println("Received from server: " + response);

        } catch (IOException e) {
            System.err.println("Error communicating with the named pipe server: " + e.getMessage());
        }
    }

    // Boilerplate code to load the mod. This is standard across all mods.
    public static ModInfo info;
    public static String modID; // Edit your pom.xml to change this
    static {
        loadModInfo();
    }
    public static final Logger logger = LogManager.getLogger(modID); // Used to output to the console.

    // This is used to prefix the IDs of various objects like cards and relics,
    // to avoid conflicts between different mods using the same name for things.
    public static String makeID(String id) {
        return modID + ":" + id;
    }

    // This will be called by ModTheSpire because of the @SpireInitializer
    // annotation at the top of the class.
    public static void initialize() {
        new PipeMod();
    }

    public PipeMod() {
        BaseMod.subscribe(this); // This will make BaseMod trigger all the subscribers at their appropriate
                                 // times.
        logger.info(modID + " subscribed to BaseMod.");
    }

    /**
     * This determines the mod's ID based on information stored by ModTheSpire.
     */
    private static void loadModInfo() {
        Optional<ModInfo> infos = Arrays.stream(Loader.MODINFOS).filter((modInfo) -> {
            AnnotationDB annotationDB = Patcher.annotationDBMap.get(modInfo.jarURL);
            if (annotationDB == null)
                return false;
            Set<String> initializers = annotationDB.getAnnotationIndex().getOrDefault(SpireInitializer.class.getName(),
                    Collections.emptySet());
            return initializers.contains(PipeMod.class.getName());
        }).findFirst();
        if (infos.isPresent()) {
            info = infos.get();
            modID = info.ID;
        } else {
            throw new RuntimeException("Failed to determine mod info/ID based on initializer.");
        }
    }
}
