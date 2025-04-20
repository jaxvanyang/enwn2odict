# Convert English WordNet to ODict

This project provides a tool to convert GWN-LMF format WordNet to ODict file
and converted files. Because WordNet is not for dictionary purposes, some
information is lost during the conversion process. But the converted dictionary
is fairly usable.

Download is available in [Releases](https://github.com/jaxvanyang/enwn2odict/releases).

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## Usage of the Tool

```bash
enwn2odict <WordNet XML> <ODict>
```

## Demos

```console
$ odict lookup enwn2odict-2024.2.odict done

────────────────────────────────
done
────────────────────────────────

Etymology #1

done

adjective

  1. having finished or arrived at completion
     ▸ certain to make history before he's done
     ▸ it's a done deed
     ▸ after the treatment, the patient is through except for follow-up
     ▸ almost through with his studies

     Notes

      a. Synonyms: done, through, through with

  2. cooked until ready to serve

Etymology #2

See also: do
```

```console
$ odict lookup enwn2odict-2024.2.odict life

────────────────────────────────
life
────────────────────────────────

life, lives

noun

  1. a characteristic state or mode of living
     ▸ social life
     ▸ city life
     ▸ real life
  2. the experience of being alive; the course of human events and activities
     ▸ he could no longer cope with the complexities of life

     Notes

      a. Synonyms: life, living

  3. the course of existence of an individual; the actions and events that occur in living
     ▸ he hoped for a new life in Australia
     ▸ he wanted to live his own life without interference from others
     ▸ get a life!
     ▸ he is trying to rebuild his life
  4. the condition of living or the state of being alive
     ▸ while there's life there's hope
     ▸ life depends on many chemical and physical processes

     Notes

      a. Synonyms: aliveness, animation, life, living

  5. the period during which something is functional (as between birth and death)
     ▸ the battery had a short life
     ▸ he lived a long and happy life

     Notes

      a. Synonyms: life, life-time, lifespan, lifetime

  6. the period between birth and the present time
     ▸ I have known him all his life
  7. the period from the present until death
     ▸ he appointed himself emperor for life
  8. a living person
     ▸ his heroism saved a life
  9. animation and energy in action or expression
     ▸ it was a heavy play and the actors tried in vain to give life to it

     Notes

      a. Synonyms: life, liveliness, spirit, sprightliness

  10. living things collectively
     ▸ the oceans are teeming with life
  11. the organic phenomenon that distinguishes living organisms from nonliving ones
     ▸ there is no life on the moon
  12. an account of the series of events making up a person's life

     Notes

      a. Synonyms: biography, life, life history, life story

  13. a motive for living
     ▸ pottery was his life
  14. a prison term lasting as long as the prisoner lives
     ▸ he got life for killing the guard

     Notes

      a. Synonyms: life, life sentence

```

## Acknowledgement

- [The Open English WordNet](https://github.com/globalwordnet/english-wordnet): the dataset.
- [Thomblin/xml_schema_generator](https://github.com/Thomblin/xml_schema_generator): save my time for parsing XML.
- [The Open Dictionary](https://odict.org): of course.

## License

This project is free to use by following the [license](LICENSE).
