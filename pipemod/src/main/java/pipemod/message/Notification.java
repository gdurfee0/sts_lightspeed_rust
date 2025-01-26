package pipemod.message;

import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonSubTypes;

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "type")
@JsonSubTypes({
        @JsonSubTypes.Type(value = CardObtained.class, name = "CardObtained"),
        @JsonSubTypes.Type(value = CardRemoved.class, name = "CardRemoved"),
        @JsonSubTypes.Type(value = Deck.class, name = "Deck"),
        @JsonSubTypes.Type(value = Gold.class, name = "Gold"),
        @JsonSubTypes.Type(value = Map.class, name = "Map"),
        @JsonSubTypes.Type(value = RelicObtained.class, name = "RelicObtained"),
        @JsonSubTypes.Type(value = Relics.class, name = "Relics"),
        @JsonSubTypes.Type(value = PotionObtained.class, name = "PotionObtained"),
        @JsonSubTypes.Type(value = Potions.class, name = "Potions"),
        @JsonSubTypes.Type(value = StartingCombat.class, name = "StartingCombat"),
        @JsonSubTypes.Type(value = EndingCombat.class, name = "EndingCombat")
})
public abstract class Notification {
    // Abstract base class for all notification types
}
